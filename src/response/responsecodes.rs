//This script represnets all code messages
use std::collections::HashMap;
use warp::http::StatusCode;

pub fn response_codes() -> HashMap<i32, (StatusCode, String, String)> {
  //code | Status code | message
  /*
  Codes between 100 - 500 are success
  Codes between 600 - ... are error
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
  
  response_msgs.insert(100,
    (
      StatusCode::OK,
      String::from("success"),
      String::from("Table was created successfully")
    )
  );
  
  //Error messages
  response_msgs.insert(600,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(600): Command not found!"),
      String::from("")
    )
  );
  response_msgs.insert(601,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(601): IO error when reading!"),
      String::from("")
    )
  );
  response_msgs.insert(602,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(602): IO error when writing!"),
      String::from("")
    )
  );
  response_msgs.insert(603,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(603): Unvalid create command"),
      String::from("You havenot probably formatted the command correctly! create <tableName> with <col> : <type> , <col2> : <type>")
    )
  );
  response_msgs.insert(604,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(604): Table not found"),
      String::from("The name of the table wasnot registered in the database file system")
    )
  );
  response_msgs.insert(605,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(605): Database type was not found!"),
      String::from("Valid types are int64, int32, float32, float64, double, byte, text, smalltext, largetext, json, binary, bool, date, time, undefined, incrementel")
    )
  );
  response_msgs.insert(606,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(606): Ip address isnot in the allowed list!"),
      String::from("Add your Ip or connect your admin to add your ip to the allowed list! ")
    )
  );
  response_msgs.insert(607,
    (
      StatusCode::NOT_FOUND,
      String::from("Err(607): Ip address is Unknown!"),
      String::from("The server doesnot know the Ip address of the client.Make sure that you havnot any proxies or firewalls that hide IP adresses at all!")
    )
  );
  response_msgs.insert(1000,
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      String::from("Err(1000) Uncaught Error while Attempting to handle the query!"),
      String::from("")
    )
  );
  response_msgs
}