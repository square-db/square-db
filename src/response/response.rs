// response.rs
use warp::reply::json;
use warp::http::StatusCode;
use std::collections::HashMap;
use crate::response::response_code:: {
  ResponseCode,
  ResponseCodeTrait,
  DataTuple
};
use crate::response::responses::Responses;
use serde_json::Value;
use crate::log::log::*;

pub struct Response;

#[derive(Debug, serde::Serialize)]
#[allow(non_snake_case)]
pub struct ResponseMessage {
  code: String,
  #[serde(serialize_with = "serialize_status_code")]
  HTTPStatusCode: StatusCode,
  msg: String,
  description: String,
  result: Option<Value>,
}

fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
S: serde::Serializer,
{
  serializer.serialize_u16(status_code.as_u16())
}

pub trait ResponseTrait {
  fn respond(result: Result<Responses, Responses>) -> warp::reply::WithStatus<warp::reply::Json>;
  fn find_by_code(code: Responses) -> ResponseMessage;
}

impl ResponseTrait for Response {
  fn find_by_code(code: Responses) -> ResponseMessage {
    let response_msgs: HashMap<Responses,
    DataTuple> = ResponseCode::response_codes();

    let binding = DataTuple::default();
    let data_tuple: &DataTuple = response_msgs.get(&code).unwrap_or(&binding);
    let (http_status_code, message, description) = &data_tuple.components();

    //log the results
    println!("[{}] {}", Log::info(*http_status_code), message.to_string());
    println!("-------------------------");

    ResponseMessage {
      code: code.to_string(),
      HTTPStatusCode: *http_status_code,
      msg: message.to_string(),
      result: None,
      description: description.to_string(),
    }

  }

  fn respond(result: Result<Responses, Responses>) -> warp::reply::WithStatus<warp::reply::Json> {
    match result.map(|code| Response::find_by_code(code.into())) {
      Ok(response_message) => {
        let json_reply = json(&response_message);
        warp::reply::with_status(json_reply, response_message.HTTPStatusCode)
      }
      Err(response_code) => {
        let response_message = Response::find_by_code(response_code.into());
        let json_reply = json(&response_message);
        warp::reply::with_status(json_reply, response_message.HTTPStatusCode)
      }
    }
  }
}