//This script is reponsible for running the cmd
use squaredb_ql::QL;
use std::collections::HashMap;
use crate::response::create_response::create_response;

pub struct Entry;

pub trait EntryTrait {
  fn new() -> Entry;
  fn handle_cmd(&self, cmd: &str) -> HashMap<String,String>;
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
  fn handle_cmd(&self, cmd: &str) -> HashMap<String,String> {
    //Change Ownership of cmd
    let checked_cmd: String = self.check_cmd(cmd);
    //cmd = PING Then user check if the server is responding correctly
    if checked_cmd == "ping" {
      // see response/responsecodes.rs
      /*
      each number has a corresponding meaning
      see reponse/reponsecodes.rs
      */
      return create_response("200", "PONG", None, None);
    }
    else {
      //use the Squaredb-ql system to activate certain commands
      return QL::init();
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