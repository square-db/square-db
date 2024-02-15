use structopt::StructOpt;
use structopt::clap:: {
  AppSettings
};
use crate::env::env:: {
  Env,
  EnvT
};
use crate::server::server:: {
  ServerT,
  Server
};
use crate::log::log::*;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "SquareDB Server", about = "The full fledged SquareDB Server", global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
pub struct Opt {

  /// Start the Server
  #[structopt(long = "start", short = "S")]
  start: bool,

  /// Entering KMS Mode
  #[structopt(long = "kms", short = "K", takes_value = true, possible_values = &["change"])]
  kms: Option<String>,

  /// Specify port [SQUARE_BIND=]
  #[structopt(long = "bind", short = "b")]
  pub bind: Option<String>,

  /// Specify password [SQUARE_PASSW=]
  #[structopt(long = "password", short = "p")]
  pub password: Option<String>,

  /// Specify user [SQUARE_USER=]
  #[structopt(long = "user", short = "u")]
  pub user: Option<String>,

  /// Specify max connections [SQUARE_MAX_CONNECTIONS=]
  #[structopt(long = "max-connections", short = "m")]
  pub max_connections: Option<String>,

  /// Specify web certificate [SQUARE_WEB_CRT=]
  #[structopt(long = "web-cert", short = "c")]
  pub web_cert: Option<String>,

  /// Specify web key [SQUARE_WEB_KEY=]
  #[structopt(long = "web-key", short = "k")]
  pub web_key: Option<String>,

  /// Specify where the data must be saved [SQUARE_DATA_FOLDER=data]
  #[structopt(long = "file", short = "f")]
  pub data_folder: Option<String>,

}

pub struct Cli;

pub trait CliT {
  fn init() -> ();
}

impl CliT for Cli {
  fn init() {
    let passed_args: Opt = Opt::from_args();
    
    //Load Env vars
    Env::init();
    Env::map_env_values_with_passed_args(passed_args.clone());
    
    //start the server
    if passed_args.start {
      if let Err(err) = Server::run() {
        println!("[{}] Error while attempting to start the server! Due to {}." , Log::error("ERR"), err);
      }
    }

    if passed_args.kms.clone().unwrap_or_else(|| String::from("")) == "change" {
      println!("Will be supproted in coming beta versions!")
    }
  }
}