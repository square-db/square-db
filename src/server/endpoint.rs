use actix_web:: {
  web,
  Responder,
  HttpRequest
};
use crate::response::response:: {
  Response,
  ResponseMessage
};
use crate::entry::entry::Entry;
use log::{error,info};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RequestParams {
  query: String,
}

pub async fn endpoint(req: HttpRequest, query_params: web::Json<RequestParams>) -> impl Responder {
  // Access the headers from the request
  let headers = req.headers();
  if let Some(val) = req.peer_addr() {
    // Retrieve the Authorization header
    match headers.get("Authorization") {
      Some(auth_header) => {
        // Convert the header value to a string slice
        let auth_str = auth_header.to_str().unwrap_or("Invalid header");
        // Auth
        // future implementation
        let response = Entry.handle_cmd(query_params.query.as_str());
        if response.status_code == 200 {
          info!("[{}] {:?}:{} - {}", response.status_code, val.ip(), query_params.query, response.result);
        } else {
          error!("[{}] {:?}:{} - {}", response.status_code, val.ip(), query_params.query, response.result);
        }
        Response::respond(response)
      },
      None => {
        let response = ResponseMessage {
          status_code: 403,
          result_type: String::from("error"),
          result: String::from("No Authorization header found.")
        };
        error!("[{}] {:?}:{} - {}", response.status_code, val.ip(), query_params.query, response.result);
        Response::respond(response)
      }
    }
  } else {
    let response = ResponseMessage {
      status_code: 403,
      result_type: String::from("error"),
      result: String::from("No IP address found.")
    };
    error!("[{}] N/A:{} - {}", response.status_code, query_params.query, response.result);
    Response::respond(response)
  }
}