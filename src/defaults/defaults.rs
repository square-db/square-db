// This script represents all default values
use crate::config::config::*;

#[allow(non_snake_case)]
pub struct Defaults;

impl Defaults {
  pub fn default_config_value() -> ConfigData {
    ConfigData {
      server: Server {
        host: String::from("127.0.0.1"),
        port: String::from("8000"),
        endpoint: String::from("db"),
        allowedIps: String::from("*")
      },
      auth: Auth {
        username: String::from("admin"),
        password: String::from("admin"),
        role: String::from("admin"),
        shareKeyRead: String::from(""),
        shareKeyWrite: String::from(""),
        shareKeyReadAndWrite: String::from(""),
      },
      engine: Engine {
        mode: String::from("dev"),
        enableDeleteTable: String::from("true"),
        enableDeleteRow: String::from("true"),
        enableForceCreation: String::from("true"),
        enableCache: String::from("true"),
        enableAutoSave: String::from("2s"),
        enableTableHashing: String::from("true")
      },
      packages: Modules {
        packages: vec!["select".to_string()]
      }
    }
  }
}