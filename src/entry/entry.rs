//This script is reponsible for running the cmd
use crate::response::response::ResponseMessage;
pub struct Entry;

impl Entry {
  /*
  This function is the main entry point to the programm it load , save and cahce commands
  */
  //All commands will be in-case-senstive!
  pub fn handle_cmd(&self, cmd: &str) -> ResponseMessage {
   //cmd = PING Then user check if the server is responding correctly
    if cmd.to_lowercase() == "" {
      return ResponseMessage{
        status_code: 200,
        result_type: String::from("success"),
        result: String::from("PONG")
      };
    }
    else {
      //use the Squaredb-ql system to activate certain commands
      return ResponseMessage{
        status_code: 200,
        result_type: String::from("success"),
        result: String::from("QL")
      };
    }
  }
}