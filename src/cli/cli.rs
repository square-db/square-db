use std::collections::HashMap;
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "SquareDB Server", about = "The full fledged SquareDB Server")]
struct Opt {
  /// Specify port
  #[structopt(long = "bind", short = "b")]
  bind: Option<String>,

  /// Specify password
  #[structopt(long = "password", short = "p")]
  password: Option<String>,

  /// Specify user
  #[structopt(long = "user", short = "u")]
  user: Option<String>,

  /// Enable environment
  #[structopt(long = "enable-env", short = "ee")]
  enable_env: bool,

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
  fn init() -> () {
    let passed_args = Opt::from_args();
    let mut global_vars: HashMap<String,
    String> = HashMap::with_capacity(10);
    if passed_args.enable_env {
      global_vars.insert("BIND".to_owned(), passed_args.bind.clone().unwrap_or_else( || env::var("SQUARE_BIND").unwrap_or_else(|_| String::from("127.0.0.1:8000"))));
      
      global_vars.insert("USER".to_owned(), passed_args.user.clone().unwrap_or_else( || env::var("SQUARE_USER").unwrap_or_else(|_| String::from("root"))));
      
      global_vars.insert("PASSW".to_owned(), passed_args.password.clone().unwrap_or_else( || env::var("SQUARE_PASSW").unwrap_or_else(|_| String::from("root"))));
      
      global_vars.insert("MAX_CONNECTIONS".to_owned(), passed_args.max_connections.clone().unwrap_or_else( || env::var("SQUARE_MAX_CONNECTIONS").unwrap_or_else(|_| String::from("root"))));
      
      global_vars.insert("TIMEOUT".to_owned(), passed_args.timeout.clone().unwrap_or_else( || env::var("SQUARE_TIMEOUT").unwrap_or_else(|_| String::from("0"))));
      
      global_vars.insert("WEB_CRT".to_owned(), passed_args.web_crt.clone().unwrap_or_else( || env::var("SQUARE_WEB_CRT").unwrap_or_else(|_| String::from(""))));
      
      global_vars.insert("WEB_KEY".to_owned(), passed_args.web_key.clone().unwrap_or_else( || env::var("SQUARE_WEB_KEY").unwrap_or_else(|_| String::from(""))));

      global_vars.insert("CLIENT_IP".to_owned(), passed_args.client_ip.clone().unwrap_or_else( || env::var("SQUARE_CLIENT_IP").unwrap_or_else(|_| String::from("*"))));
    }
  }
}