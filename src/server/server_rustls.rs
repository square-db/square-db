use rustls_pemfile::certs;
use rustls_pemfile::pkcs8_private_keys;
use rustls::pki_types::PrivateKeyDer;
use rustls::pki_types::PrivatePkcs8KeyDer;
use rustls::pki_types::CertificateDer;
use rustls::ServerConfig;
use actix_governor::Governor;
use actix_governor::GovernorConfigBuilder;
use actix_web::web;
use actix_web::HttpServer;
use actix_web::App;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::process;
use std::fs::File;
use std::io::BufReader;
use crate::server::endpoint::endpoint;
use crate::err::error::ServerError;
use log::{error,info};

pub struct ServerTLS;

impl ServerTLS {
  fn check_variables(env_vars: &HashMap<String, String>) -> Result<(),
  ServerError> {
    // Typo: should be `ServerError` instead of `ServerError::RequiredArgsError`
    let required_vars = ["BIND",
      "DATA_FOLDER", "WEB_KEY", "WEB_CERT", "ENDPOINT", "PASSW", "USER"];
    for var in required_vars {
      if env_vars[var].is_empty() {
        return Err(ServerError::RequiredArgsError(var.to_string())); // Added `to_string()` to convert `&str` to `String`
      }
    }
    Ok(())
  }

  #[actix_web::main]
  pub async fn run(env_vars: HashMap<String, String>) -> Result<(),
  ServerError> {
    if let Err(err) = ServerTLS::check_variables(&env_vars) {
      return Err(err);
    }

    let max_connections: usize = match env_vars["MAX_CONNECTIONS"].parse() {
      Ok(val) => val,
      Err(_) => {
        return Err(ServerError::MaxConnectionsError); // Placeholder value for Error enum variant
      }
    };
    
    let workers: usize = match env_vars["WORKERS"].parse() {
      Ok(val) => val,
      Err(_) =>{ return Err(ServerError::WorkersNumberError);}
    };
    
    if workers == 0 {
      return Err(ServerError::WorkersNumberError);
    }
    
    let socket_addr: SocketAddr = env_vars["BIND"].parse().expect("Failed to parse Socket Address");
    info!("[SERVER][TLS] running on {}.", socket_addr);

    let governor_conf = GovernorConfigBuilder::default()
    .per_second(3)
    .burst_size(20)
    .finish()
    .unwrap();

    let mut builder = HttpServer::new(move || {
      App::new().wrap(Governor::new(&governor_conf)).route("/", web::post().to(endpoint))
    })
    .workers(workers)
    .max_connections(max_connections);

    if let (Ok(key_file), Ok(cert_file)) = (File::open(&env_vars["WEB_KEY"]), File::open(&env_vars["WEB_CERT"])) {
      let key_reader = &mut BufReader::new(key_file);
      let key = pkcs8_private_keys(key_reader).unwrap_or_else(|err| {
        error!("[SERVER][TLS] Failed to read key file: {}", err);
        process::exit(1);
      })[0].clone();
      let pkcs8_key = PrivatePkcs8KeyDer::from(key);
      let private_key_der = PrivateKeyDer::from(pkcs8_key);

      let cert_reader = &mut BufReader::new(cert_file);
      let cert = certs(cert_reader).unwrap_or_else(|err| {
        error!("[SERVER][TLS] Failed to read cert file: {}", err);
        process::exit(1);
      });
      let mut cert_der = Vec::new();
      for c in cert {
        let c_vec = c.to_owned();
        let c_der = CertificateDer::from(c_vec);
        cert_der.push(c_der);
      }

      let tls_config = ServerConfig::builder()
      .with_no_client_auth()
      .with_single_cert(cert_der, private_key_der)
      .unwrap_or_else(|err| {
        error!("[SERVER][TLS] Failed to build server config: {}", err);
        process::exit(1);
      });

      info!("[SERVER][TLS] TLS is used!\n");
      builder = builder.bind_rustls_0_22(socket_addr, tls_config).unwrap_or_else(|err| {
        error!("[SERVER][TLS] Failed to bind address with rustls: {}", err);
        process::exit(1);
      });
    } else {
      error!("[SERVER][TLS] Failed to open key or cert file");
      process::exit(1);
    }

    Ok(builder.run().await.unwrap())
  }
}