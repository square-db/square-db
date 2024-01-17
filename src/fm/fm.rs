use crate::response::responses::{Responses};
use crate::response::process_responses::ProcessResponses;

use std::fs:: {
  self,
  File
};
use std::io:: {
  Read,
  Write
};

pub struct FileManager;

pub trait FileManagerTrait {
  fn read_file_to_string(path: String) -> Result<String,Responses>;
  fn write_file_incremental(path: String, content: &String) -> Result<(),
  Responses>;
}

impl FileManagerTrait for FileManager {
  fn read_file_to_string(path: String) -> Result<String,Responses> {
    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(_) => {
        return Err(Responses::Process(ProcessResponses::ReadingErr));
      }
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
      Ok(_) => Ok(buffer),
      Err(_) => {
        Err(Responses::Process(ProcessResponses::ReadingErr))
      }
    }
  }

  fn write_file_incremental(path: String, content: &String) -> Result<(),Responses> {
    let parent_dir = match std::path::Path::new(&path).parent() {
      Some(parent) => parent,
      None => {
        return Err(Responses::Process(ProcessResponses::WritingErr));
      }
    };

    if !parent_dir.exists() {
      match fs::create_dir_all(parent_dir) {
        Ok(_) => {}
        Err(_) => {
          return Err(Responses::Process(ProcessResponses::WritingErr));
        }
      }
    }

    let mut file = match File::create(&path) {
      Ok(file) => file,
      Err(_) => {
        return Err(Responses::Process(ProcessResponses::WritingErr));
      }
    };

    for chunk in content.as_bytes().chunks(1024) {
      match file.write_all(chunk) {
        Ok(_) => continue,
        Err(_) => {
          return Err(Responses::Process(ProcessResponses::WritingErr));
        }
      }
    }

    Ok(())
  }
}