use crate::log::log::*;
use structopt::StructOpt;
use structopt::clap:: {
  AppSettings,
  arg_enum
};
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
#[structopt(name = "SquareDB Server", about = "The full fledged SquareDB Server", global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
pub struct Opt {

  /// Start the Server
  #[structopt(long = "start" , short="S")]
  start: bool,

  /// Entering KMS Mode
  #[structopt(long = "kms", short="K", takes_value = true, possible_values = &["generate_key", "change"])]
  kms: String,


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


    if passed_args.start {ß11
      //Load Env vars
      Env::init();
      Env::map_env_values_with_passed_args(passed_args.clone());
      //check for key validlity
      key();
      //start the server
      Server::run();
    }

    if passed_args.kms == "generate_key" {
    println!("[{}] PUB_KEY: {}", Log::info("INFO"), Key::generate_valid_pub_key());
    }else if passed_args.kms == "change" {
      println!("Will be supproted in coming beta versions!")
    }

  }
}