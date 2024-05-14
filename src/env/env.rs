use dotenv::dotenv;
use std::collections::HashMap;
use crate::cli::cli::Opt;
use std::env;
use log::{error,info};

pub struct Env;

impl Env {
  pub fn init() {
    match dotenv() {
      Ok(_) => info!("[ENV] Loaded .env successfully"),
      Err(e) => {
        info!("[ENV] Cannot load .env");
        error!("[ENV] {}", e);
      }
    }
  }

  pub fn get_env_values(passed_args: Opt) -> HashMap<String,
  String> {
    let mut env_vars = HashMap::new();

    macro_rules! insert {
      ($map: ident, $key: expr, $field: expr, $env_var: expr, $default_value: expr) => {
        let value = $field.unwrap_or_else( || {
          env::var($env_var).unwrap_or_else(|_| {
            info!("[ENV] Using default value for {}", $key);
            $default_value.to_string().clone()
          })
        });
        $map.insert($key.to_owned(), value);
      };
    }

    insert!(env_vars, "BIND", passed_args.bind, "SQUARE_BIND", "127.0.0.1:8000");
    insert!(env_vars, "ENDPOINT", passed_args.endpoint, "SQUARE_ENDPOINT", "db");
    insert!(env_vars, "USER", passed_args.user, "SQUARE_USER", "root");
    insert!(env_vars, "PASSW", passed_args.password, "SQUARE_PASSW", "root");
    insert!(env_vars, "MAX_CONNECTIONS", passed_args.max_connections, "SQUARE_MAX_CONNECTIONS", "0");
    insert!(env_vars, "WORKERS", passed_args.workers, "SQUARE_WORKERS", "2");
    insert!(env_vars, "WEB_KEY", passed_args.web_key, "SQUARE_WEB_KEY", "");
    insert!(env_vars, "WEB_CERT", passed_args.web_cert, "SQUARE_WEB_CERT", "");
    insert!(env_vars, "DATA_FOLDER", passed_args.data_folder, "SQUARE_DATA_FOLDER", "data");
    insert!(env_vars, "ENC", passed_args.encryption, "SQUARE_ENC", "");

    env_vars
  }
}