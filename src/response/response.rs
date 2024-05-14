use actix_web:: {
  HttpResponse,
  http::StatusCode
};

pub struct Response;

#[derive(Debug, serde::Serialize)]
pub struct ResponseMessage {
  pub status_code: u16,
  pub result_type: String,
  pub result: String
}

impl Response {
  pub fn respond(result: ResponseMessage) -> HttpResponse {
    let json_reply = HttpResponse::build(StatusCode::from_u16(result.status_code).unwrap())
    .json(result);
    json_reply
  }
}