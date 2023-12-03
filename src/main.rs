mod config;
mod defaults;
mod log;
use std::net::{SocketAddr, IpAddr};
use log::log::*;
use config::config:: {
  ConfigStruct,
  ConfigTrait
};
use warp::Filter;

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
  /*
  read Configuration file
  */
  let config_file = ConfigStruct::new();
  //Server : This is main entry point 👉 of the whole software
  //Define the HTTP Method as pots for security reasons
  let api = warp::post()
  //Main endpoint
  //changeable through configuration file and --endpoint
  .and(warp::path(config_file.server.endpoint))
  //Turn the sended body as json
  .and(warp::body::json())
  //Define all params and enable serde on it
  .map(|params: RequestParams| {
    //Testing if the RequestParams were sended successfully
    println!("{:?}", params.username);
    println!("{:?}", params.password);
    println!("{:?}", params.query);
    //reply with the same params in the mean time
    warp::reply::json(&params)
  });

  //Run the Server
  //host and port are changeable through the configuration file
  let host_ip: IpAddr = config_file.server.host.parse().expect(&Log::error("Cannot parse host ip address"));
  let socket_addr = SocketAddr::new(host_ip, config_file.server.port.parse::<u16>().unwrap());
  println!("{} on {}" , Log::success("Server runned successfully") , Log::success(socket_addr));

  warp::serve(api).run(socket_addr).await;
}