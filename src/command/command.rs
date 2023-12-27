//This script will represnet all exsiting commands in the software
//This script will represnet all exsiting commands in the software
use std::collections::HashMap;
use crate::operation::createcmd::{Create};
use crate::operation::operation::OperationT;

pub struct Command;

pub trait CommandTrait {
  fn check(cmd: String) -> Result<i32,
  i32>;
  fn run(cmd: String) -> Result<i32,
  i32>;
}

impl CommandTrait for Command {
  //check if command exsist
  fn check(cmd: String) -> Result<i32,
  i32> {
    //available built-in cmds
    let mut cmds: HashMap<String,
    String> = HashMap::new();
    //cmd Description
    cmds.insert(String::from("create"), String::from("create <tableName> : create and prepare Table for Usage!"));
    if let Some(_) = cmds.get(&cmd) {
      Ok(0)
    } else {
      Err(1)
    }
  }
  
  //run command
  fn run(cmd: String) -> Result<i32,
  i32>{
    let command: String = cmd.split_whitespace().next().map(|s| s.to_string()).unwrap_or_default();

    if command == "create" {
      return Create::new().run(cmd);
    }
    //unnecessery
    Err(600)
  }
}