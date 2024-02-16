use rustls_pemfile:: {
  certs,
  pkcs8_private_keys
};
use rustls::pki_types:: {
  PrivateKeyDer,
  PrivatePkcs8KeyDer,
  CertificateDer
};
use rustls::ServerConfig;
use crate::response::create_response::create_response;
use actix_governor:: {
  Governor,
  GovernorConfigBuilder
};
use actix_web:: {
  web,
  Responder,
  HttpRequest,
  HttpServer,
  App
};

use std::
{
  collections::HashMap,
  net::SocketAddr,
  process,
  fs::File,
  io::BufReader
};
use crate:: {
  env::env:: {
    Env,
    EnvT
  },
  log::log:: {
    Log,
    LogTrait
  },
  response::response:: {
    ResponseTrait,
    Response
  },
  entry::entry:: {
    EntryTrait,
    Entry
  }
};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RequestParams {
  query: String,
}

async fn endpoint(req: HttpRequest, query_params: web::Json<RequestParams>) -> impl Responder {
  // Access the headers from the request
  let headers = req.headers();
  if let Some(val) = req.peer_addr() {
    println!("{:?}", val.ip());
    // Retrieve the Authorization header
    match headers.get("Authorization") {
      Some(auth_header) => {
        // Convert the header value to a string slice
        let auth_str = auth_header.to_str().unwrap_or("Invalid header");
        //Auth
        //future implmentation
        println!("-------------------------");
        Response::respond(Entry::new().handle_cmd(query_params.query.as_str()))
      },
      None => Response::respond(create_response("403", "No Authorization header found.", None, None))
    }
  } else {
    Response::respond(create_response("403", "No Ip Adress found.", None, None))
  }
}

pub struct Server;

pub trait ServerT {
  fn check_variables(env_vars: &HashMap<String, String>);
  fn run() -> std::io::Result<()>;
}

impl ServerT for Server {
  fn check_variables(env_vars: &HashMap<String, String>) {
    if env_vars["BIND"].is_empty() {
      println!("[{}] BIND cannot be empty! Specify it using --bind <bind> or set SQUARE_BIND in the enviroment", Log::error("ERR"));
      process::exit(1);
    }

    if env_vars["DATA_FOLDER"].is_empty() {
      println!("[{}] DATA_FOLDER cannot be empty! Specify it using --file <data folder> or set SQUARE_DATA_FOLDER in the enviroment", Log::error("ERR"));
      process::exit(1);
    }

    if env_vars["WEB_KEY"].is_empty() {
      println!(
        "[{}] {} is empty",
        Log::info("INFO"),
        Log::info("WEB_KEY")
      );
    }

    if env_vars["WEB_CERT"].is_empty() {
      println!(
        "[{}] {} is empty",
        Log::info("INFO"),
        Log::info("WEB_CERT")
      );
    }

  }

  #[actix_web::main]
  async fn run() -> std::io::Result<()> {
    let env_vars: HashMap<String,
    String> = Env.get_env_vars_from_session();

    let max_connections: usize = match env_vars["MAX_CONNECTIONS"].parse() {
      Ok(val) => val,
      Err(_) => {
        println!("[{}]: MAX_CONNECTIONS must be a valid positive integer.", Log::error("ERR"));
        process::exit(1);
      }
    };

    Self::check_variables(&env_vars);

    let socket_addr: SocketAddr = env_vars["BIND"].parse().expect("Failed to parse Socket Address");
    println!("[{}] Server running on {}.", Log::success("SUCCESS"), Log::success(socket_addr));

    let governor_conf = GovernorConfigBuilder::default()
    .per_second(3)
    .burst_size(20)
    .finish()
    .unwrap();

    let mut builder = HttpServer::new(move || {
      App::new().wrap(Governor::new(&governor_conf)).route("/", web::post().to(endpoint))
    })
    .max_connections(max_connections);

    if !env_vars["WEB_KEY"].is_empty() && !env_vars["WEB_CERT"].is_empty() {
      if let (Ok(key_file), Ok(cert_file)) = (File::open(&env_vars["WEB_KEY"]), File::open(&env_vars["WEB_CERT"])) {
        let key_reader = &mut BufReader::new(key_file);
        let key = pkcs8_private_keys(key_reader).unwrap_or_else(|err| {
          println!("[{}] Failed to read key file: {}",Log::error("ERR"),err);
          process::exit(1);
        })[0].clone();
        let pkcs8_key = PrivatePkcs8KeyDer::from(key);
        let private_key_der = PrivateKeyDer::from(pkcs8_key);

        let cert_reader = &mut BufReader::new(cert_file);
        let cert = certs(cert_reader).unwrap_or_else(|err| {
          println!("[{}] Failed to read cert file: {}",Log::error("ERR"), err);
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
          println!("[{}] Failed to build server config: {}",Log::error("ERR"), err);
          process::exit(1);
        });

        println!("[{}] TLS is used!\n", Log::info("INFO"));
        builder = builder.bind_rustls_0_22(socket_addr, tls_config).unwrap_or_else(|err| {
          println!("[{}] Failed to bind address with rustls: {}",Log::error("ERR"),err);
          process::exit(1);
        });
      } else {
        println!("[{}] Failed to open key or cert file",Log::error("ERR"));
        process::exit(1);
      }
    } else {
      builder = builder.bind(socket_addr).unwrap_or_else(|err| {
        println!("[{}] Failed to bind address: {}", Log::error("ERR"), err);
        process::exit(1);
      });
    }

    builder.run().await
  }
}