//This script achivates certain commands
//It is responsible parsing -> Operate -> saving
use crate::command::command:: {
  Command,
  CommandTrait
};
use crate::log::log::*;

pub struct Activate;

pub trait ActivateTrait {
  fn new() -> Activate;
  fn activate(&self, cmd: String) -> Result<i32,
  i32>;
}

impl ActivateTrait for Activate {
  fn new() -> Activate {
    Activate {}
  }
  
  fn activate(&self, cmd: String) -> Result<i32,
  i32> {
    
    //get only the command alone
    let command: String = cmd.split_whitespace().next().map(|s| s.to_string()).unwrap_or_default();

    if let Ok(_) = Command::check(command.clone()) {
     Command::run(cmd) // the whole statement
    }else {
      //command not found
      println!("{}" , Log::error("Err(600) Command not found! "));
      Err(600)
    }
  }
}