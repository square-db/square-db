use dotenv::dotenv;
use crate::log::log::*;
use std::collections::HashMap;
use crate::cli::cli::Opt;
use std::env;
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
  fn map_env_values_with_passed_args(passed_args: Opt) -> ();
}

impl EnvT for Env {
  fn init() -> () {
    match dotenv() {
      Ok(_) => println!("[{}] Loaded .env successfully", Log::info("INFO")),
      Err(e) => {
        println!("[{}] Cannot load .env", Log::info("INFO"));
        println!("[{}] {}", Log::error("ERR"), e);
      }
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
      "ENDPOINT" => "db",
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
  fn get_env_vars_from_session(&self) -> HashMap<String,
  String> {
    SessionManager::get(String::from("env"))
    .unwrap_or_else( || Self::default())
  }
  fn map_env_values_with_passed_args(passed_args: Opt) -> () {
    let default_env_vars: HashMap<String,
    String> = Self::default();

    let mut global_vars: HashMap<String,
    String> = HashMap::with_capacity(8);

    macro_rules! insert {
      ($map: expr, $key: expr, $field: expr, $env_var: expr, $default_value: expr) => {
        let value = $field.clone().unwrap_or_else( || {
          let default_value = env::var($env_var)
          .unwrap_or_else(|_| {
            println!("[{}] Using default value for {}", Log::info("INFO"), Log::info($key));
            String::from($default_value.clone())
          });
          default_value
        });

        $map.insert($key.to_owned(), value);
      };
    }

    insert!(global_vars, "BIND", passed_args.bind, "SQUARE_BIND", default_env_vars["BIND"]);
    insert!(global_vars, "ENDPOINT", passed_args.endpoint, "SQUARE_ENDPOINT", default_env_vars["ENDPOINT"]);
    insert!(global_vars, "USER", passed_args.user, "SQUARE_USER", "root");
    insert!(global_vars, "PASSW", passed_args.password, "SQUARE_PASSW", "root");
    insert!(global_vars, "MAX_CONNECTIONS", passed_args.max_connections, "SQUARE_MAX_CONNECTIONS", "0");
    insert!(global_vars, "TIMEOUT", passed_args.timeout, "SQUARE_TIMEOUT", "0");
    insert!(global_vars, "WEB_CRT", passed_args.web_crt, "SQUARE_WEB_CRT", "");
    insert!(global_vars, "WEB_KEY", passed_args.web_key, "SQUARE_WEB_KEY", "");
    insert!(global_vars, "CLIENT_IP", passed_args.client_ip, "SQUARE_CLIENT_IP", "*");

    SessionManager::set(String::from("env"), global_vars);
  }
}