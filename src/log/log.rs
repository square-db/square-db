pub struct Log;

pub trait LogTrait {
  fn error<T: std::fmt::Display>(msg: T) -> String;
  fn warning<T: std::fmt::Display>(msg: T) -> String;
  fn success<T: std::fmt::Display>(msg: T) -> String;
  fn info<T: std::fmt::Display>(msg: T) -> String;
}

impl LogTrait for Log {
  fn error<T: std::fmt::Display>(msg: T) -> String {
    format!("\x1b[31m{}\x1b[0m", msg) // ANSI escape code for red text
  }

  fn warning<T: std::fmt::Display>(msg: T) -> String {
    format!("\x1b[33m{}\x1b[0m", msg) // ANSI escape code for yellow text
  }

  fn success<T: std::fmt::Display>(msg: T) -> String {
    format!("\x1b[32m{}\x1b[0m", msg) // ANSI escape code for green text
  }

  fn info<T: std::fmt::Display>(msg: T) -> String {
    format!("\x1b[36m{}\x1b[0m", msg) // ANSI escape code for cyan text
  }
}