//This script represnets all code messages
use std::collections::HashMap;
use warp::http::StatusCode;

pub fn response_codes() -> HashMap<i32, (StatusCode, String, String)> {
  //code | Status code | message
  /*
  Codes between 100 - 500 are success
  q                sE = Succes but can cause future Errors
  sW = Succes but can cause future Warnings
  Codes between 500 - 600 are warnings
  wp = warning while parsing
  wM = Module warning
  wP = Process warning
  Codes between 600 - ... are error
  f = fatal error
  p = parsing error
  P = process error
  M = Module error
  0                SU = none ignoreable Security Risk error
  */
  /*
  Messages formatt : Type Status : msg;
  */
  let mut response_msgs: HashMap<i32,
  (StatusCode, String, String)> = HashMap::new();
  //code HTTPSTATUSCODE MSG Description
  response_msgs.insert(0,
    (
      StatusCode::OK,
      String::from("PONG"),
      String::from("")
    )
  );
  response_msgs.insert(1000,
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      String::from("(p|P) Err(1000) Uncaught Error while Attempting to handle the query!"),
      String::from("")
    )
  );
  response_msgs
}