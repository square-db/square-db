use dotenv::dotenv;
use crate::log::log::*;
use std::collections::HashMap;
use crate::session::session:: {
  SessionManager,
  SessionManagerT
};

pub struct Env;

pub trait EnvT {
  fn init() -> ();
  fn default() -> HashMap<String,
  String>;
  fn get_env_vars_from_session(&self) -> HashMap<String,
  String>;
}

impl EnvT for Env {
  fn init() -> () {
    match dotenv() {
      Ok(_) => println!("[{}] Loaded .env successfully", Log::info("INFO")),
      Err(_) => println!("[{}] Cannot load .env", Log::info("INFO"))
    }
  }
  fn default() -> HashMap<String,
  String> {
    let default_env_vars: HashMap<String,
    String> = HashMap::with_capacity(8);

    macro_rules! insert {
      ($map: expr, $($key: expr => $value: expr), *) => {
        {
          let mut map = $map;
          $(
            map.insert($key.to_string(), String::from($value));
          )*
          map
        }
      };
    }

    let default_env_vars = insert!(
      default_env_vars,
      "BIND" => "127.0.0.1:8000",
      "USER" => "root",
      "PASSW" => "root",
      "MAX_CONNECTIONS" => "0",
      "TIMEOUT" => "0",
      "WEB_CRT" => "",
      "WEB_KEY" => "",
      "CLIENT_IP" => "*"
    );

    default_env_vars
  }
  
  fn get_env_vars_from_session(&self) -> HashMap<String,String> {
    SessionManager::get(String::from("env"))
    .unwrap_or_else( || Self::default())
  }
}