#![deny(clippy::mem_forget)]
#![forbid(unsafe_code)]

mod entry;
mod env;
mod cli;
mod server;
mod response;
mod err;
mod panic;
////////////
use cli::cli::Cli;
use env_logger::Env;
use panic::panic::set_panic;

fn main(){
println!(r#"
 _____                            _____  ____
/ ____|                          |  __ \|  _ \
| (___   __ _ _   _  __ _ _ _____| |  | | |_) |
\___ \ / _` | | | |/ _` | '__/ _ \ |  | |  _  <
____) | (_| | |_| | (_| | | |  __/ |__| | |_) |
|_____/ \__, |\__,_|\__,_|_|  \___|_____/|____/
           | |
           |_|
"#);
set_panic();
//Init the logger
env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
//Init the cli
Cli::init();
}