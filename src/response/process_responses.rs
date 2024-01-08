use std::fmt;

#[derive(Eq, PartialEq, Hash)]
pub enum ProcessResponses {
  ConnectivitySuccess,
  QueryUnknowErr,
  DecryptionErr,
  IpAddressUnknownErr,
  IpAddressUnallowedErr,
  WritingErr,
  ReadingErr,
  UncaughtError
}

#[derive(Eq, PartialEq, Hash)]
pub enum DatatypeResponses {
  DataTypeUnknown
}

impl fmt::Display for ProcessResponses {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result_str = match self {
      ProcessResponses::ConnectivitySuccess => "Connectivity Success",
      ProcessResponses::QueryUnknowErr => "Query Unknown Error",
      ProcessResponses::DecryptionErr => "Decryption Error",
      ProcessResponses::IpAddressUnknownErr => "IP Address Unknown Error",
      ProcessResponses::IpAddressUnallowedErr => "IP Address Unallowed Error",
      ProcessResponses::WritingErr => "Writing Error",
      ProcessResponses::ReadingErr => "Reading Error",
      ProcessResponses::UncaughtError => "Uncaught Error",
    };
    write!(f, "{}", result_str)
  }
}

impl fmt::Display for DatatypeResponses {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let result_str = match self {
      DatatypeResponses::DataTypeUnknown => "Data Type Unknown",
    };
    write!(f, "{}", result_str)
  }
}