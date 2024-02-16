#![deny(clippy::mem_forget)]
#![forbid(unsafe_code)]
/*
extern crate rcgen;
use rcgen::generate_simple_self_signed;
*/
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
/*
// Generate a certificate that's valid for "localhost" and "hello.world.example"
let subject_alt_names = vec!["hello.world.example".to_string(),
	"localhost".to_string()];

let cert = generate_simple_self_signed(subject_alt_names).unwrap();
println!("{}", cert.serialize_pem().unwrap());
println!("{}", cert.serialize_private_key_pem());
*/
//Init the cli
Cli::init();
}