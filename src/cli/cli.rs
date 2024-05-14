use structopt::StructOpt;
use structopt::clap::AppSettings;
use crate::env::env::Env;
use crate::server::server::Server;
use crate::server::server_rustls::ServerTLS;
use log::error;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "SquareDB Server", about = "The full fledged SquareDB Server", global_settings = &[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])]
pub struct Opt {

  ///Start the Server
  #[structopt(long = "start", short = "S")]
  pub start: bool,

  ///Enable TLS Mode
  #[structopt(long = "tls", short = "t")]
  pub tls: bool,
  
  ///Enable Encryption [Donot set a value if it is unwanted]
  #[structopt(long = "encryption", short = "E")]
  pub encryption: Option<String>,
  

  ///Port [SQUARE_BIND=]
  #[structopt(long = "bind", short = "b")]
  pub bind: Option<String>,
  
  ///Endpoint [SQUARE_ENDPOINT=]
  #[structopt(long = "endpoint", short = "e")]
  pub endpoint: Option<String>,

  ///Password [SQUARE_PASSW=]
  #[structopt(long = "password", short = "p")]
  pub password: Option<String>,

  ///User [SQUARE_USER=]
  #[structopt(long = "user", short = "u")]
  pub user: Option<String>,

  ///Max connections [SQUARE_MAX_CONNECTIONS=]
  #[structopt(long = "max-connections", short = "m")]
  pub max_connections: Option<String>,
  
  ///Max connections [SQUARE_WORKERS=]
  #[structopt(long = "workers", short = "w")]
  pub workers: Option<String>,

  ///Web certificate [SQUARE_WEB_CRT=]
  #[structopt(long = "web-cert", short = "c")]
  pub web_cert: Option<String>,

  ///Web key [SQUARE_WEB_KEY=]
  #[structopt(long = "web-key", short = "k")]
  pub web_key: Option<String>,

  ///Data folder [SQUARE_DATA_FOLDER=]
  #[structopt(long = "file", short = "f")]
  pub data_folder: Option<String>,
  
  ///Log level (info,error,debug)
  #[structopt(long = "RUST_LOG")]
  pub log_level: Option<String>

}

pub struct Cli;

impl Cli {
  pub fn init() {
    let passed_args: Opt = Opt::from_args();
    //Load Env vars
    Env::init();

    //start the server
    if passed_args.start {
      if passed_args.tls == true {
        if let Err(err) = ServerTLS::run(Env::get_env_values(passed_args.clone())) {
          error!("{}", err);
        }
      } else {
        if let Err(err) = Server::run(Env::get_env_values(passed_args.clone())) {
          error!("{}", err);
        }
      }
    }
    
  }
}