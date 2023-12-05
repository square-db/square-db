pub struct Entry {
  mode: String
}

pub trait EntryTrait {
  fn new(mode: &str) -> Entry;
  fn handle_cmd(&self, cmd: &str) -> Result<i32,
  i32>;
  fn check_cmd(&self, cmd: &str) -> String;
}

impl EntryTrait for Entry {
  fn new(mode: &str) -> Entry {
    Entry {
      mode: mode.to_string()
    }
  }
  /*
  This function is the main entry point to the programm it load , save and cahce commands
  */
  //All commands will be in-case-senstive!
  fn handle_cmd(&self, cmd: &str) -> Result<i32,
  i32> {
    //Change Ownership of cmd
    let mut checked_cmd: String = self.check_cmd(cmd);
    //cmd = PING Then user check if the server is responding correctly
    if checked_cmd == "ping" {
      // see response/responsecodes.rs
      /*
      each number has a corresponding meaning
      */
      return Ok(0)
    }
    Err(1000)
  }

  /*
  This function is responsible for parsing the programm and checking security risks
  */
  fn check_cmd(&self, cmd: &str) -> String {
    // lowercase the strings
    // strings between \u must not be lowercased
    //There is not any comments signatures to insure nore security for the programm
    let result = cmd
    .split("#[u]")
    .enumerate()
    .flat_map(|(index, part)| {
      if index % 2 == 0 {
        vec![part.to_lowercase()]
      } else {
        vec![part.to_string()]
      }
    })
    .collect::<Vec<String>>()
    .join("");

    String::from(result)
  }
}