use crate::operation::operation:: {
  OperationT,
  ParsedCmd
};
//use crate::table::table::Table;
/*use crate::load::disk:: {
  Disk,
  DiskT
};*/
use crate::response::responses::{Responses};
use crate::response::cmd_responses::CreateCmdResponses;
//use crate::datatypes::datatypes::DatabaseValue;

pub struct Create;

impl OperationT<Create> for Create {
  fn new() -> Create {
    Create {}
  }
  fn run(&self, _cmd: String) -> Result<Responses,Responses>{
    return Ok(Responses::CreateCmd(CreateCmdResponses::Success))
  }
  fn parse(&self, _cmd: String) -> ParsedCmd {
    ParsedCmd::new()
  }
  fn validate(&self, _cmd: String) -> Result<Responses,Responses> {
    return Ok(Responses::CreateCmd(CreateCmdResponses::Success))
  }
}