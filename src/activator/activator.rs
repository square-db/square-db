//This script achivates certain commands
//It is responsible for loading -> parsing -> Operate -> saving by request
use crate::config::config::Engine;
//use crate::command::command::{Command,CommandTrait};

pub struct Activate {
  settings: Engine
}

pub trait ActivateTrait {
  fn config(settings: Engine) -> Activate;
  fn activate(&self , cmd: String) -> Result<i32,
  i32>;
}

impl ActivateTrait for Activate {
  fn config(settings: Engine) -> Activate {
    Activate {
      settings : settings
    }
  }
  fn activate(&self , cmd: String) -> Result<i32,
  i32> {
    Ok(0)
  }
}