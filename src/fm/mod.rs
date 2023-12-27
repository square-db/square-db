pub mod fm {
  use std::fs:: {
    self,
    File
  };
  use std::io:: {
    Read,
    Write
  };
  use crate::log::log::*;
  pub struct FileManager;

  pub trait FileManagerTrait {
    fn read_file_to_string(path: &str) -> Result<String,
    i32>;
    fn write_file_incremental(path: String, content: &String) -> Result<(),
    i32>;
  }

  impl FileManagerTrait for FileManager {
    fn read_file_to_string(path: &str) -> Result<String,
    i32> {
      let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
          println!("{} {}", Log::error("Err(601) Error opening file:"), Log::error(path));
          return Err(601);
        }
      };

      let mut buffer = String::new();
      match file.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => {
          eprintln!("{} {}", Log::error("Err(601) Error reading file:"), Log::error(path));
          Err(601)
        }
      }
    }

    fn write_file_incremental(path: String, content: &String) -> Result<(),
    i32> {
      let parent_dir = match std::path::Path::new(&path).parent() {
        Some(parent) => parent,
        None => {
          println!("{} {}", Log::error("Err(602) Invalid path:"), Log::error(path));
          return Err(602);
        }
      };

      if !parent_dir.exists() {
        match fs::create_dir_all(parent_dir) {
          Ok(_) => {}
          Err(_) => {
            println!("{} {}", Log::error("Err(602) Error creating directories for:"), Log::error(path));
            return Err(602);
          }
        }
      }

      let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => {
          println!("{} {}", Log::error("Err(602) Error creating file:"), Log::error(path));
          return Err(602);
        }
      };

      for chunk in content.as_bytes().chunks(1024) {
        match file.write_all(chunk) {
          Ok(_) => continue,
          Err(_) => {
            println!("{} {}", Log::error("Err(602) Error writing to file:"), Log::error(path));
            return Err(602);
          }
        }
      }

      Ok(())
    }
  }
}