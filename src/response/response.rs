//This script must handle the diffrent succes and error codes
//It will be not located in defaults/ ut schueve more organization in the software
use warp::reply::json;
use warp::http::StatusCode;
use crate::response::responsecodes::response_codes;
use serde_json::Value;
pub struct Response;

#[derive(Debug, serde::Serialize)]
#[allow(non_snake_case)]
pub struct ResponseMessage {
  //code | Status code | message
  // (Type) Err?(1000) msg
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
  code: i32,
  #[serde(serialize_with = "serialize_status_code")]
  HTTPStatusCode: StatusCode,
  msg: String,
  description: String ,// further description 
  result : Option<Value>// for returning data by select cmds
}

//Avoinding serializing HTTPStatusCodes
fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

pub trait ResponseTrait {
  fn respond(result: Result<i32, i32>) -> warp::reply::WithStatus<warp::reply::Json>;
  fn find_by_code(code: i32) -> ResponseMessage;
}

impl ResponseTrait for Response {
  fn find_by_code(code: i32) -> ResponseMessage {
    let response_msgs = response_codes();
    if let Some(data) = response_msgs.get(&code) {
      let (http_status_code, message , description) = data;
      ResponseMessage {
        code : code,
        HTTPStatusCode : *http_status_code,
        msg : message.to_string(),
        result : None,
        description : description.to_string()
      }
    }else{
      ResponseMessage {
        code : -100,
        HTTPStatusCode : StatusCode::INTERNAL_SERVER_ERROR,
        msg : String::from("(f|M) ERR(-100) : Cannot find valid Response Message"),
        result : None,
        description : String::from("This is likely due tgat there is not any corresponding value for the gives code!")
      }
    }
  }
  //The respond main Function
  fn respond(result: Result<i32, i32>) -> warp::reply::WithStatus<warp::reply::Json> {
    //Determine if that must be an Error or a Sccuess branch
    match result {
      Ok(code) => {
        let response_message = Response::find_by_code(code);
        // Create the JSON reply
        let json_reply = json(&response_message);
        // Set the desired HTTP status code
        warp::reply::with_status(json_reply, StatusCode::OK)
      }
      Err(code) => {
        let response_message = Response::find_by_code(code);
        // Create the JSON reply
        let json_reply = json(&response_message);
        // Set the desired HTTP status code
        warp::reply::with_status(json_reply, StatusCode::BAD_REQUEST)
      }
    }
  }
}