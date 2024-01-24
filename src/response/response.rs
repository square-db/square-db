// response.rs
use warp::reply::json;
use warp::http::StatusCode;
use std::collections::HashMap;
use crate::log::log::*;

pub struct Response;

#[derive(Debug, serde::Serialize)]
#[allow(non_snake_case)]
pub struct ResponseMessage {
  StatusCode: String,
  msg: String,
  description: String,
  result: String,
}

pub trait ResponseTrait {
  fn respond(result: HashMap<String, String>) -> warp::reply::WithStatus<warp::reply::Json>;
  fn create_response_message(result: HashMap<String, String>) -> ResponseMessage;
}

impl ResponseTrait for Response {
  fn create_response_message(result:
    HashMap<String, String>) -> ResponseMessage {
    //log the results
    println!("[{}] {}", Log::info(result["StatusCode"].clone()), result["msg"].clone());
    println!("-------------------------");

    ResponseMessage {
      StatusCode: result["StatusCode"].clone(),
      msg: result["msg"].clone(),
      result: result["result"].clone(),
      description: result["description"].clone(),
    }

  }

  fn respond(result: HashMap<String, String>) -> warp::reply::WithStatus<warp::reply::Json> {
    let response_message = Response::create_response_message(result);
    let json_reply = json(&response_message);
    let http_status_code: StatusCode = response_message.StatusCode.parse().unwrap_or_else(|_| StatusCode::OK);
    warp::reply::with_status(json_reply, http_status_code)
  }
}