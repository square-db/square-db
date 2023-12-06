//This script is reposible for preparing the cmd
use crate::config::config::Engine;
use crate::activator::activator:: {
  Activate,
  ActivateTrait
};
pub struct Entry {
  settings: Engine
}

pub trait EntryTrait {
  fn new(settings: Engine) -> Entry;
  fn handle_cmd(&self, cmd: &str) -> Result<i32,
  i32>;
  fn check_cmd(&self, cmd: &str) -> String;
}

impl EntryTrait for Entry {
  fn new(settings: Engine) -> Entry {
    Entry {
      settings: settings
    }
  }
  /*
  This function is the main entry point to the programm it load , save and cahce commands
  */
  //All commands will be in-case-senstive!
  fn handle_cmd(&self, cmd: &str) -> Result<i32,
  i32> {
    //Change Ownership of cmd
    let checked_cmd: String = self.check_cmd(cmd);
    //cmd = PING Then user check if the server is responding correctly
    if checked_cmd == "ping" {
      // see response/responsecodes.rs
      /*
      each number has a corresponding meaning
      see reponse/reponsecodes.rs
      */
      return Ok(0)
    }else {
      //use the Activator to activate certain commands
      Activate::config(self.settings.clone()).
      activate(checked_cmd.clone());
    }
    Err(1000)
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