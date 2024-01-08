//This script is reposible for preparing the cmd
use crate::response::responses::{Responses};
use crate::response::process_responses::ProcessResponses;
use crate::activator::activator:: {
  Activate,
  ActivateTrait
};
pub struct Entry;

pub trait EntryTrait {
  fn new() -> Entry;
  fn handle_cmd(&self, cmd: &str) -> Result<Responses,Responses>;
  fn check_cmd(&self, cmd: &str) -> String;
}

impl EntryTrait for Entry {
  fn new() -> Entry {
    Entry {}
  }
  /*
  This function is the main entry point to the programm it load , save and cahce commands
  */
  //All commands will be in-case-senstive!
  fn handle_cmd(&self, cmd: &str) -> Result<Responses,Responses> {
    //Change Ownership of cmd
    let checked_cmd: String = self.check_cmd(cmd);
    //cmd = PING Then user check if the server is responding correctly
    if checked_cmd == "ping" {
      // see response/responsecodes.rs
      /*
      each number has a corresponding meaning
      see reponse/reponsecodes.rs
      */
      return Ok(Responses::Process(ProcessResponses::ConnectivitySuccess))
    }
    else if checked_cmd == "eip1"{
      return Err(Responses::Process(ProcessResponses::IpAddressUnallowedErr))
    }
    else if checked_cmd == "eip2"{
      return Err(Responses::Process(ProcessResponses::IpAddressUnknownErr))
    }
    else {
      //use the Activator to activate certain commands
      return Activate::new().activate(checked_cmd.clone());
    }
  }

  /*
  This function is responsible for parsing the programm and checking security risks
  */
  fn check_cmd(&self, cmd: &str) -> String {
    // Remove semicolons at the beginning and end of the string
    let cmd_trimmed = cmd.trim_matches(';');

    // Lowercase the strings between '\u', leaving them unchanged
    let result = cmd_trimmed
    .split("#[u]")
    .enumerate()
    .flat_map(|(index, part)| {
      if index % 2 == 0 {
        vec![part.to_lowercase()]
      } else {
        vec![part.to_string()]
      }
    })
    .collect::<Vec<String>>()
    .join("");

    String::from(result)
  }

}