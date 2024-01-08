use std::fmt;

#[derive(Eq, PartialEq, Hash)]
pub enum CreateCmdResponses {
  Success,
  UnvalidCommand
}

impl fmt::Display for CreateCmdResponses {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result_str = match self {
      CreateCmdResponses::Success => "Success",
      CreateCmdResponses::UnvalidCommand => "Invalid Command",
    };
    write!(f, "{}", result_str)
  }
}