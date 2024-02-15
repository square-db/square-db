#![deny(clippy::mem_forget)]
#![forbid(unsafe_code)]

mod log;
mod entry;
mod env;
mod cli;
mod session;
mod server;
mod response;
////////////
use cli::cli:: {
Cli,
CliT
};

fn main(){
println!(r#"
 _____                            _____  ____
/ ____|                           |  __ \|  _ \
| (___   __ _ _   _  __ _ _ _____| |  | | |_) |
\___ \ / _` | | | |/ _` | '__/ _ \ |  | |  _  <
____) | (_| | |_| | (_| | | |  __/ |__| | |_) |
|_____/ \__, |\__,_|\__,_|_|  \___|_____/|____/
           | |
           |_|
"#);
//Init the cli
Cli::init();
}