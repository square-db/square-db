//This script represnets all code messages
use std::collections::HashMap;
use std::default::Default;
use warp::http::StatusCode;
use crate::response::responses::Responses;
use crate::response::process_responses::ProcessResponses;
use crate::response::cmd_responses::CreateCmdResponses;
use crate::response::process_responses::DatatypeResponses;
use crate::response::process_responses::KmsResponses;

pub struct DataTuple ((StatusCode, String, String));
impl Default for DataTuple {
  fn default() -> DataTuple{
    // Provide default values for each component of the tuple
    DataTuple((StatusCode::OK, String::default(), String::default()))
  }
}
impl DataTuple {
    // Public method to access the components
    pub fn components(&self) -> &(StatusCode, String, String) {
        &self.0
    }
}

pub struct ResponseCode;

pub trait ResponseCodeTrait {
  fn response_codes() -> HashMap<Responses,
  DataTuple>;

}

impl ResponseCodeTrait for ResponseCode {
  fn response_codes() -> HashMap<Responses,
  DataTuple> {
    let mut response_msgs: HashMap<Responses,DataTuple> = HashMap::with_capacity(22);
    //default
    response_msgs.insert(Responses::DefaultDataTuple, Default::default());
    //connectivity Messages
    response_msgs.insert(
      Responses::Process(ProcessResponses::ConnectivitySuccess),
      DataTuple((
        StatusCode::OK,
        String::from("PONG"),
        String::from("")
      ))
    );

    //create command Responses
    response_msgs.insert(
      Responses::CreateCmd(CreateCmdResponses::Success),
      DataTuple((
        StatusCode::OK,
        String::from("success"),
        String::from("Table was created successfully")
      ))
    );

    //Data Type System responses
    response_msgs.insert(
      Responses::DataType(DatatypeResponses::DataTypeUnknown),
      DataTuple((
        StatusCode::NOT_FOUND,
        String::from("Err(605): Database type was not found!"),
        String::from("Valid types are int64, int32, float32, float64, double, byte, text, smalltext, largetext, json, binary, bool, date, time, undefined, incrementel")
      ))
    );
    //Kms Responses
    response_msgs.insert(
      Responses::Kms(KmsResponses::KeyNotfound),
      DataTuple((
        StatusCode::NOT_FOUND,
        String::from("Err(KMS): Key is invalid!"),
        String::from("")
      ))
    );
    //Process Responses
    response_msgs.insert(
      Responses::Process(ProcessResponses::QueryUnknowErr),
      DataTuple((
        StatusCode::NOT_FOUND,
        String::from("Err(600): Command not found!"),
        String::from("")
      ))
    );
    response_msgs.insert(
      Responses::Process(ProcessResponses::ReadingErr),
      DataTuple((
        StatusCode::NOT_FOUND,
        String::from("Err(601): IO error when reading!"),
        String::from("")
      ))
    );
    response_msgs.insert(
      Responses::Process(ProcessResponses::WritingErr),
      DataTuple((
        StatusCode::NOT_FOUND,
        String::from("Err(602): IO error when writing!"),
        String::from("")
      ))
    );
    response_msgs.insert(
      Responses::CreateCmd(CreateCmdResponses::UnvalidCommand),
      DataTuple((
        StatusCode::NOT_FOUND,
        String::from("Err(603): Unvalid create command"),
        String::from("You have not formatted the command correctly! create <tableName> with <col> : <type> , <col2> : <type>")
      ))
    );
    response_msgs.insert(
      Responses::Process(ProcessResponses::IpAddressUnallowedErr),
      DataTuple((
        StatusCode::FORBIDDEN,
        String::from("Err(606): Ip address is not in the allowed list!"),
        String::from("Add your IP or connect your admin to add your IP to the allowed list!")
      ))
    );
    response_msgs.insert(
      Responses::Process(ProcessResponses::IpAddressUnknownErr),
      DataTuple((
        StatusCode::FORBIDDEN,
        String::from("Err(607): Ip address is Unknown!"),
        String::from("The server does not know the IP address of the client. Make sure that you haven't any proxies or firewalls that hide IP addresses at all!")
      ))
    );
    response_msgs.insert(
      Responses::Process(ProcessResponses::DecryptionErr),
      DataTuple((
        StatusCode::FORBIDDEN,
        String::from("Err(608): Decryption error key is unknown"),
        String::from("")
      ))
    );
    response_msgs.insert(
      Responses::Process(ProcessResponses::UncaughtError),
      DataTuple((
        StatusCode::INTERNAL_SERVER_ERROR,
        String::from("Err(1000) Uncaught Error while Attempting to handle the query!"),
        String::from("")
      ))
    );

    response_msgs
  }
}