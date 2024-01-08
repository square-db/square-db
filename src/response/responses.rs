use crate::response::process_responses:: {
  DatatypeResponses,
  ProcessResponses
};
use crate::response::cmd_responses:: {
  CreateCmdResponses
};

#[derive(Eq, Hash, PartialEq)]
pub enum Responses {
  DefaultDataTuple,
  Process(ProcessResponses),
  DataType(DatatypeResponses),
  CreateCmd(CreateCmdResponses),
}

impl ToString for Responses {
  fn to_string(&self) -> String {
    match self {
      Responses::DefaultDataTuple => todo!(),
      Responses::Process(process_responses) => process_responses.to_string(),
      Responses::DataType(datatype_responses) => datatype_responses.to_string(),
      Responses::CreateCmd(create_cmd_responses) => create_cmd_responses.to_string(),
    }
  }
}