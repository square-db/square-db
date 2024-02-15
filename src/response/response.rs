use actix_web:: {
  HttpResponse,
  http::StatusCode
};
use std::collections::HashMap;
use crate::log::log::*;

pub struct Response;
#[derive(Debug, serde::Serialize)]
#[allow(non_snake_case)]
pub struct ResponseMessage {
  StatusCode: u16,
  // Changed from String to u16 for HTTP status codes
  msg: String,
  description: String,
  result: String,
}

pub trait ResponseTrait {
  fn respond(result: HashMap<String, String>) -> HttpResponse;
  fn create_response_message(result: HashMap<String, String>) -> ResponseMessage;
}

impl ResponseTrait for Response {
  fn create_response_message(result: HashMap<String, String>) -> ResponseMessage {
    // Log the results
    println!("[{}] {}", Log::info(result["StatusCode"].clone()), result["msg"].clone());
    println!("-------------------------\n");

    ResponseMessage {
      // Parse the status code as u16
      StatusCode: result["StatusCode"].parse().unwrap_or(200),
      msg: result["msg"].clone(),
      result: result["result"].clone(),
      description: result["description"].clone(),
    }
  }

  fn respond(result: HashMap<String, String>) -> HttpResponse {
    let response_message = Response::create_response_message(result);
    let json_reply = HttpResponse::build(StatusCode::from_u16(response_message.StatusCode).unwrap())
    .json(response_message);
    json_reply
  }
}