#![deny(clippy::mem_forget)]
#![forbid(unsafe_code)]

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
mod session;
mod server;
////////////
use cli::cli:: {
  Cli,
  CliT
};

fn main(){
  println!(r#"
   _____                            _____  ____  
  / ____|                          |  __ \|  _ \ 
 | (___   __ _ _   _  __ _ _ __ ___| |  | | |_) |
  \___ \ / _` | | | |/ _` | '__/ _ \ |  | |  _ < 
  ____) | (_| | |_| | (_| | | |  __/ |__| | |_) |
 |_____/ \__, |\__,_|\__,_|_|  \___|_____/|____/ 
            | |                                  
            |_|                                  
"#);
  //Init the cli
  Cli::init();
}