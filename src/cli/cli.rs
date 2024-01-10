use std::collections::HashMap;
use std::env;
use crate::log::log::*;
use structopt::StructOpt;
use crate::env::env:: {
  Env,
  EnvT
};
use crate::server::server:: {
  ServerT,
  Server
};
use crate::session::session:: {
  SessionManager,
  SessionManagerT
};

#[derive(Debug, StructOpt)]
#[structopt(name = "SquareDB Server", about = "The full fledged SquareDB Server")]
struct Opt {
  /// Start the Server
  #[structopt(long = "start", short = "s")]
  start: bool,

  /// Specify port
  #[structopt(long = "bind", short = "b")]
  bind: Option<String>,

  /// Specify password
  #[structopt(long = "password", short = "p")]
  password: Option<String>,

  /// Specify user
  #[structopt(long = "user", short = "u")]
  user: Option<String>,

  /// Specify max connections
  #[structopt(long = "max-connections", short = "mc")]
  max_connections: Option<String>,

  /// Specify timeout
  #[structopt(long = "timeout", short = "t")]
  timeout: Option<String>,

  /// Specify web certificate
  #[structopt(long = "web-crt", short = "crt")]
  web_crt: Option<String>,

  /// Specify web key
  #[structopt(long = "web-key", short = "key")]
  web_key: Option<String>,

  /// Specify client IP
  #[structopt(long = "client-ip", short = "i")]
  client_ip: Option<String>,
}

pub struct Cli;

pub trait CliT {
  fn init() -> ();
}

impl CliT for Cli {
  fn init() {
    let default_env_vars: HashMap<String,
    String> = Env::default();
    let passed_args: Opt = Opt::from_args();
    let mut global_vars: HashMap<String,
    String> = HashMap::with_capacity(8);

    macro_rules! insert {
      ($map: expr, $key: expr, $field: expr, $env_var: expr, $default_value: expr) => {
        let value = $field.clone().unwrap_or_else( || {
          let default_value = env::var($env_var)
          .unwrap_or_else(|_| {
            println!("[{}] Using default value for {}",Log::info("INFO"), Log::info($key));
            String::from($default_value.clone())
          });
          default_value
        });

        $map.insert($key.to_owned(), value);
      };
    }

    insert!(global_vars, "BIND", passed_args.bind, "SQUARE_BIND", default_env_vars["BIND"]);
    insert!(global_vars, "USER", passed_args.user, "SQUARE_USER", "root");
    insert!(global_vars, "PASSW", passed_args.password, "SQUARE_PASSW", "root");
    insert!(global_vars, "MAX_CONNECTIONS", passed_args.max_connections, "SQUARE_MAX_CONNECTIONS", "0");
    insert!(global_vars, "TIMEOUT", passed_args.timeout, "SQUARE_TIMEOUT", "0");
    insert!(global_vars, "WEB_CRT", passed_args.web_crt, "SQUARE_WEB_CRT", "");
    insert!(global_vars, "WEB_KEY", passed_args.web_key, "SQUARE_WEB_KEY", "");
    insert!(global_vars, "CLIENT_IP", passed_args.client_ip, "SQUARE_CLIENT_IP", "*");

    SessionManager::set(String::from("env"), global_vars);

    if passed_args.start {
      Server::run();
    }
  }
}