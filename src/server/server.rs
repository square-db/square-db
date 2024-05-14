use std::process;
use actix_governor::Governor;
use actix_governor::GovernorConfigBuilder;
use actix_web::web;
use actix_web::HttpServer;
use actix_web::App;
use std::collections::HashMap;
use std::net::SocketAddr;
use crate::server::endpoint::endpoint;
use crate::err::error::ServerError;
use log::{error,info};

pub struct Server;

impl Server {
  fn check_variables(env_vars: &HashMap<String, String>) -> Result<(),
  ServerError> {
    // Typo: should be `ServerError` instead of `ServerError::RequiredArgsError`
    let required_vars = ["BIND","DATA_FOLDER","ENDPOINT", "PASSW", "USER"];
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
    if let Err(err) = Server::check_variables(&env_vars) {
      return Err(err);
    }

    let max_connections: usize = match env_vars["MAX_CONNECTIONS"].parse() {
      Ok(val) => val,
      Err(_) =>{ return Err(ServerError::MaxConnectionsError);}
    };
    
    let workers: usize = match env_vars["WORKERS"].parse() {
      Ok(val) => val,
      Err(_) =>{ return Err(ServerError::WorkersNumberError);}
    };
    if workers == 0 {
      return Err(ServerError::WorkersNumberError);
    }
    let socket_addr: SocketAddr = env_vars["BIND"].parse().expect("Failed to parse Socket Address");
    info!("[SERVER] running on {}.", socket_addr);

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

    builder = builder.bind(socket_addr).unwrap_or_else(|err| {
      error!("[SERVER] Failed to bind address: {}", err);
      process::exit(1);
    });

    Ok(builder.run().await.unwrap())
  }
}