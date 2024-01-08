//This script achivates certain commands
//It is responsible parsing -> Operate -> saving
use crate::response::responses::{Responses};
use crate::response::process_responses::ProcessResponses;
use crate::command::command:: {
  Command,
  CommandTrait
};


pub struct Activate;

pub trait ActivateTrait {
  fn new() -> Activate;
  fn activate(&self, cmd: String) -> Result<Responses,Responses>;
}

impl ActivateTrait for Activate {
  fn new() -> Activate {
    Activate {}
  }
  
  fn activate(&self, cmd: String) -> Result<Responses,Responses>{
    
    //get only the command alone
    let command: String = cmd.split_whitespace().next().map(|s| s.to_string()).unwrap_or_default();

    if let Ok(_) = Command::check(command.clone()) {
     Command::run(cmd) // the whole statement
    }else {
      //command not found
      Err(Responses::Process(ProcessResponses::QueryUnknowErr))
    }
  }
}