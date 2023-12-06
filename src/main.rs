mod config;
mod defaults;
mod log;
mod response;
mod entry;
mod activator;
mod command;
////////////
use std::net:: {
  SocketAddr,
  IpAddr
};
use std::env;
use log::log::*;
use config::config:: {
  ConfigStruct,
  ConfigTrait
};
use response::response:: {
  Response,
  ResponseTrait
};
use warp::Filter;
use entry::entry:: {
  Entry,
  EntryTrait
};
//Define the RequestParams
//All data must be sended as Strings
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RequestParams {
  username: String,
  password: String,
  query: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  println!("   _____                            ");
  println!("  / ____|                           ");
  println!(" | (___   __ _ _   _  __ _ _ __ ___ ");
  println!("  \\___ \\ / _` | | | |/ _` | '__/ _ \\");
  println!("  ____) | (_| | |_| | (_| | | |  __/");
  println!(" |_____/ \\__, |\\__,_|\\__,_|_|  \\___|");
  println!("           | |                       ");
  println!("          _|_|_  ____                ");
  println!("         |  __ \\|  _ \\               ");
  println!(" ______  | |  | | |_) |              ");
  println!("|______| | |  | |  _ <               ");
  println!("          | |__| | |_) |              ");
  println!("          |_____/|____/               ");

  /*config file path*/
  let mut config_file_path: String = String::from("config.toml");
  /*reading passed arguments*/
  let env: Vec<String> = env::args().collect();
  // Checking if at least two arguemnts were passed
  // [ "root/script"  , "--config" , "<config:Path>"]
  if env.len() >= 3 {
    //if --config was passed then set the config file path to this path
    if env[1] == "--config" {
      config_file_path = env[2].to_string();
    }
  }
  /*
  read Configuration file
  */
  let config_file = ConfigStruct::new(&config_file_path);

  //Server : This is main entry point 👉 of the whole software
  //Define the HTTP Method as pots for security reasons
  let api = warp::post()
  //Main endpoint
  //changeable through configuration file and --endpoint
  .and(warp::path(config_file.server.endpoint))
  //Turn the sended body as json
  .and(warp::body::json())
  //Define all params and enable serde on it
  .map(move |params: RequestParams| {
    //anthor config struct
    let config_engine = ConfigStruct::new(&config_file_path);
    //Testing if the RequestParams were sended successfully
    println!("{:?}", params.username);
    println!("{:?}", params.password);
    println!("{:?}", params.query);

    return Response::respond(Entry::new(config_engine.engine.clone()).handle_cmd(&params.query));
  });

  //Run the Server
  //host and port are changeable through the configuration file
  let host_ip: IpAddr = config_file.server.host.parse().expect(&Log::error("Cannot parse host ip address"));
  let socket_addr = SocketAddr::new(host_ip, config_file.server.port.parse::<u16>().unwrap());
  println!("{} on {}", Log::success("Server runned successfully"), Log::success(socket_addr));

  warp::serve(api).run(socket_addr).await;
}