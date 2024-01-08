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
////////////
use std::net:: {
  SocketAddr,
  IpAddr
};
use crate::log::log::*;
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
  //Server : This is main entry point 👉 of the whole software
  //Define the HTTP Method as post for security reasons
  let api = warp::post()
  //Main endpoint
  //changeable through configuration file and --endpoint
  .and(warp::path("db"))
  //Turn the sended body as json
  .and(warp::body::json())
  .and(warp::addr::remote())
  .map(move |params: RequestParams, addr: Option<SocketAddr>| {
    let ip_address: std::net::SocketAddr;
    if let Some(ip) = addr {
      ip_address = ip;
      println!("-------------------------");
      println!("\n{} {}\n", &ip_address, &params.query);
        return Response::respond(Entry::new().handle_cmd(&params.query));
    }
    else {
      return Response::respond(Entry::new().handle_cmd("EIP2"));
    }
  });

  //Run the Server
  //host and port are changeable through the configuration file
  let host_ip: IpAddr = String::from("127.0.0.1").parse().expect(&Log::error("Cannot parse host ip address"));
  let socket_addr = SocketAddr::new(host_ip, String::from("8000").parse::<u16>().unwrap());
  println!("{} on {}", Log::success("Server runned successfully"), Log::success(socket_addr));

  warp::serve(api).run(socket_addr).await;
}