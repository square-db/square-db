use crate::operation::operation:: {
  OperationT,
  ParsedCmd
};
use crate::table::table::Table;
use crate::load::disk:: {
  Disk,
  DiskT
};
use crate::log::log::*;
use crate::datatypes::datatypes::DatabaseValue;

pub struct Create;

impl OperationT<Create> for Create {
  fn new() -> Create {
    Create {}
  }


  fn run(&self, cmd: String) -> Result<i32,
  i32> {
    let parsed_cmd: ParsedCmd = self.parse(cmd.clone());
    if let Err(code) = self.validate(cmd.clone()) {
      return Err(code);
    }
    let mut table: Table = Table::new();
    table.name = parsed_cmd.clone().table_name;

    for (key, value) in parsed_cmd.parts.iter() {
      if key.trim() == "id"{
        println!("[INFO] {}" , Log::info("Column 'id' processing forbidden"));
        continue;
      }
      let parts: Vec<&str> = value.split('|').collect();
      //checking data values
      for data_type in &parts {
        if let Err(code) = DatabaseValue::check(data_type.to_string()){
          println!("{} `{}` {}" , Log::error("Datatype of"), data_type , Log::error("was not found!"));
          return Err(code);
        }
      }
      table.columns.insert(
        key.to_string(),
        {
          let cols: Vec<String> = parts.iter().map(|s| s.to_string()).collect();
          cols
        }
      );
    }
    match Disk::write_table(table) {
      Ok(_) => {
        println!("Success: {}", Log::success("Table was created successfully"));
        return Ok(100);
      },
      Err(e) => {
        return Err(e);
      }
    }
  }


  fn parse(&self, cmd: String) -> ParsedCmd {
    let mut parsed_cmd = ParsedCmd::new();

    let tokens: Vec<&str> = cmd.split_whitespace().collect();

    if tokens.len() >= 4 && tokens[0].eq_ignore_ascii_case("create") {
      parsed_cmd.table_name = String::from(tokens[1]);

      let mut i = 3; // Skip "with" and start processing pairs

      while i + 2 < tokens.len() {
        let name = String::from(tokens[i]);
        let type_ = String::from(tokens[i + 2]);
        parsed_cmd.parts.insert(name, type_);

        i += 4; // Move to the next pair
      }
    }
    parsed_cmd
  }
  fn validate(&self, cmd: String) -> Result<i32,
  i32> {
    let tokens: Vec<&str> = cmd.split_whitespace().collect();

    if tokens.len() >= 6 && tokens[0].eq_ignore_ascii_case("create") && tokens[2].eq_ignore_ascii_case("with") {
      // Remove commas from the slice and create a new slice
      // Check if commas and colons are positioned correctly
      let token_parts: Vec<&str> = tokens[3..].to_vec();

      let is_valid_syntax = token_parts.iter().enumerate().all(|(i, &token)| {
        if i % 4 == 1 {
          token == ":" && i + 1 < tokens.len() // Ensure there is a token after colon
        } else if i % 4 == 3 {
          token == "," && i + 1 < tokens.len() // Ensure there is a token after comma
        } else {
          true // Allow other tokens
        }
      }) && token_parts.last().map_or(true, |&last| last != ",");

      if is_valid_syntax {
        return Ok(0);
      } else {
        return Err(603);
      }
    } else {
      Err(603) // Unknown command or invalid syntax
    }
  }
}