use crate:: {
  log::log::*,
  response::create_response::create_response,
  env::env:: {
    Env,
    EnvT
  },
  response::response:: {
    Response,
    ResponseTrait
  },
  entry::entry:: {
    Entry,
    EntryTrait
  },
};
use std:: {
  collections::HashMap,
  net::SocketAddr,
  process
};
use warp::Filter;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RequestParams {
  query: String,
}

pub struct Server;

pub trait ServerT {
  fn check_variables(env_vars: &HashMap<String, String>);
  fn run();
}

impl ServerT for Server {
  fn check_variables(env_vars: &HashMap<String, String>) {
    if env_vars["ENDPOINT"].is_empty() {
      println!("[{}] Endpoint cannot be empty!", Log::error("ERR"));
      process::exit(1);
    }

    if env_vars["WEB_CRT"].is_empty() && !env_vars["WEB_KEY"].is_empty() || !env_vars["WEB_CRT"].is_empty() && env_vars["WEB_KEY"].is_empty() {
      println!(
        "[{}] {} is empty, but {} is not.",
        Log::info("INFO"),
        Log::info("WEB_CRT"),
        Log::info("WEB_KEY")
      );
    }
  }

  #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
  async fn run() {
    let env_vars: HashMap<String,
    String> = Env.get_env_vars_from_session();
    Self::check_variables(&env_vars);

    let endpoint = &env_vars["ENDPOINT"];
    let api = warp::post()
    .and(warp::path(endpoint.clone()))
    .and(warp::body::json())
    .and(warp::header::optional("Authorization"))
    .and(warp::addr::remote())
    .map(
      move |params: RequestParams, auth_header: Option<String>, addr: Option<SocketAddr>| {
        println!("-------------------------");
        if let Some(auth_header) = auth_header {
          println!("[{}] {}", Log::info(addr.unwrap()), Log::info(&params.query));
          return Response::respond(Entry::new().handle_cmd(&params.query));
        } else {
          println!("[{}] No Authorization header found", Log::info(addr.unwrap()));
          return Response::respond(create_response(
            "403",
            "Authentication Required",
            None,
            Some("No data was found in the Authorization header!"),
          ));
        }
      },
    );

    let socket_addr: SocketAddr = env_vars["BIND"].parse().expect("Failed to parse Socket Address");

    println!(
      "[{}] Server runned on on {}/{}",
      Log::success("SUCCESS"),
      Log::success(socket_addr),
      Log::success(&env_vars["ENDPOINT"])
    );

    if !env_vars["WEB_CRT"].is_empty() && !env_vars["WEB_KEY"].is_empty() {
      println!("[{}] TLS used...", Log::info("INFO"));
      warp::serve(api)
      .tls()
      .cert_path(&env_vars["WEB_CRT"])
      .key_path(&env_vars["WEB_KEY"])
      .run(socket_addr)
      .await;
    } else {
      warp::serve(api).run(socket_addr).await;
    }
  }
}