mod log;
mod response;
mod entry;
mod activator;
mod command;
mod operation;
mod load;
mod fm;
mod table;
mod datatypes;
mod encryptor;
mod env;
mod cli;
////////////
use crate::env::env:: {
  Env,
  EnvT
};
use cli::cli:: {
  Cli,
  CliT
};

fn main(){
  //Load Env vars
  Env::init();
  //Init the cli
  Cli::init();
}