use std::collections::HashMap;
use crate::response::responses::{Responses};

#[derive(Debug , Clone)]
pub struct ParsedCmd {
  pub table_name : String,
  pub parts : HashMap<String,String>
}

impl ParsedCmd {
  pub fn new() -> ParsedCmd{
    ParsedCmd{
      table_name : String::from(""),
      parts : {
        let  map : HashMap<String,String> = HashMap::new();
        map
      }
    }
  }
}

pub trait OperationT<T> {
  fn new() -> T;
  fn run(&self , cmd: String) -> Result<Responses, Responses>;
  fn parse(&self , cmd: String) -> ParsedCmd;
  fn validate(&self , cmd: String) -> Result<Responses,Responses>;
}