use crate::log::log::*;
use structopt::StructOpt;
use crate::encryptor::key::key;
use crate::encryptor::key:: {
  KeyTrait,
  Key
};
use crate::env::env:: {
  Env,
  EnvT
};
use crate::server::server:: {
  ServerT,
  Server
};

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "SquareDB Server", about = "The full fledged SquareDB Server")]
pub struct Opt {
  /// Start the Server
  #[structopt(long = "start", short = "s")]
  start: bool,

  /// Restore all issued KEYs
  #[structopt(long = "restore")]
  restore: bool,

  /// Generates a valid Public key
  #[structopt(long = "gen-key")]
  generate_key: bool,

  /// Cache
  #[structopt(long = "cache")]
  cache: bool,

  /// Specify port
  #[structopt(long = "bind", short = "b")]
  pub bind: Option<String>,

  /// Specify Endpoint
  #[structopt(long = "endpoint", short = "e")]
  pub endpoint: Option<String>,

  /// Specify password
  #[structopt(long = "password", short = "p")]
  pub password: Option<String>,

  /// Specify user
  #[structopt(long = "user", short = "u")]
  pub user: Option<String>,

  /// Specify max connections
  #[structopt(long = "max-connections", short = "mc")]
  pub max_connections: Option<String>,

  /// Specify timeout
  #[structopt(long = "timeout", short = "t")]
  pub timeout: Option<String>,

  /// Specify web certificate
  #[structopt(long = "web-crt", short = "crt")]
  pub web_crt: Option<String>,

  /// Specify web key
  #[structopt(long = "web-key", short = "key")]
  pub web_key: Option<String>,

  /// Specify client IP
  #[structopt(long = "client-ip", short = "i")]
  pub client_ip: Option<String>,
}

pub struct Cli;

pub trait CliT {
  fn init() -> ();
}

impl CliT for Cli {
  fn init() {
    let passed_args: Opt = Opt::from_args();

    Env::map_env_values_with_passed_args(passed_args.clone());

    if passed_args.start {
      //Load Env vars
      Env::init();
      //check for key validlity
      key();
      Server::run();
    }

    if passed_args.generate_key {
      println!("[{}] {} keep these key secret and never expose it! IF THIS KEY WENT LOST YOU (CAN) RESTORE IT", Log::info("INFO"), Key::generate_valid_pub_key());
    }

    if passed_args.restore {
      Server::run();
    }

    if passed_args.cache {
      println!("Still coming in other versions 1.0.0-beta22")
    }
  }
}