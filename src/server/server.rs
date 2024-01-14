use crate::log::log::*;
use warp::Filter;
use std::process;
use std::collections::HashMap;
use std::net::SocketAddr;
use crate::env::env:: {
  Env,
  EnvT
};
use crate::response::response:: {
  Response,
  ResponseTrait
};
use crate::entry::entry:: {
  Entry,
  EntryTrait
};


//Define the RequestParams
//All data must be sended as Strings
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RequestParams {
  username: String,
  password: String,
  query: String,
}


pub struct Server;
pub trait ServerT {
  fn run() -> ();
}

impl ServerT for Server {
  #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
  async fn run() -> () {
    let env_vars: HashMap<String,
    String> = Env.get_env_vars_from_session();
    //Define the HTTP Method as post for security reasons
    let endpoint = &env_vars["ENDPOINT"];
    if endpoint.is_empty() {
      println!("[{}] Endpoint cannot be empty!", Log::error("ERR"));
      process::exit(1);
    }
    let api = warp::post()
    //Main endpoint
    //changeable through configuration file and --endpoint
    .and(warp::path(endpoint.clone()))
    //Turn the sended body as json
    .and(warp::body::json())
    .and(warp::addr::remote())
    .map(move |params: RequestParams, addr: Option<SocketAddr>| {
      println!("{}", &params.query);
      let ip_address: std::net::SocketAddr;
      if let Some(ip) = addr {
        ip_address = ip;
        println!("-------------------------");
        println!("\n{} {}\n", &ip_address, &params.query);
        return Response::respond(Entry::new().handle_cmd(&params.query));
      }
      else {
        return Response::respond(Entry::new().handle_cmd("EIP2"));
      }
    });

    //Run the Server
    //host and port are changeable through the configuration file
    let socket_addr: SocketAddr = match env_vars["BIND"].parse() {
      Ok(addr) => addr,
      Err(_) => {
        println!("[{}] Cannot parse socket address {}", Log::error("ERR"), Log::error(&env_vars["BIND"]));
        panic!("Failed to parse Socket Address");
      }
    };
    
    if env_vars["WEB_CRT"].is_empty() && !env_vars["WEB_KEY"].is_empty() {
      println!("[{}] {} is empty, but {} is not.", Log::info("INFO"), Log::info("WEB_CRT"), Log::info("WEB_"));
    } else if !env_vars["WEB_CRT"].is_empty() && env_vars["WEB_KEY"].is_empty() {
      println!("[{}] {} is empty, but {} is not.", Log::info("INFO"), Log::info("WEB_KEY"), Log::info("WEB_CRT"));
    } else {
      println!("[{}] TLS used...", Log::info("INFO"));
    }
    
    
    println!("[{}] Server runned on on {}/{}", Log::success("SUCCESS"), Log::success(socket_addr), Log::success(&env_vars["ENDPOINT"]));
    if !env_vars["WEB_CRT"].is_empty() && !env_vars["WEB_KEY"].is_empty() {
      warp::serve(api)
      .tls()
      .cert_path(&env_vars["WEB_CRT"])
      .key_path(&env_vars["WEB_KEY"])
      .run(socket_addr)
      .await;
    } else {
      warp::serve(api)
      .run(socket_addr)
      .await;
    }
  }
}