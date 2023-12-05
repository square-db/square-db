// This script is to read configuration file
use toml::de::from_str;
use crate::defaults::defaults::{Defaults};
use crate::log::log::*;
use std::fs;

pub struct ConfigStruct;

pub trait ConfigTrait {
  fn new(config_file_path : &String) -> ConfigData;
}

#[derive(Debug, serde::Deserialize)]
pub struct ConfigData {
  pub server: Server,
  pub auth: Auth,
  pub engine: Engine,
  pub packages: Modules
}

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct Server {
  pub host: String,
  pub port: String,
  pub endpoint: String,
  pub allowedIps: String
}

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct Auth {
  pub username: String,
  pub password: String,
  pub role: String,
  pub shareKeyRead: String,
  pub shareKeyWrite: String,
  pub shareKeyReadAndWrite: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct Engine {
  pub mode: String,
  pub enableDeleteTable: String,
  pub enableDeleteRow: String,
  pub enableForceCreation: String,
  pub enableCache: String,
  pub enableAutoSave: String,
  pub enableTableHashing: String
}

#[derive(Debug, serde::Deserialize)]
pub struct Modules {
  pub packages: Vec<String>
}

impl ConfigTrait for ConfigStruct {
  fn new(config_file_path: &String) -> ConfigData {
    // Default values
    let default_values : ConfigData = Defaults::default_config_value();
    
    // Read config.toml
    let toml_str = match fs::read_to_string(config_file_path) {
      Ok(content) => content,
      Err(_) => {
        // Cannot read config.toml
        println!("{}" , Log::warning("Failed to read config.toml. Using default values."));
        return default_values;
      }
    };
    
    // Parse the config.toml
    let result: Result<ConfigData, toml::de::Error> = from_str(&toml_str);

    match result {
      Ok(config) => {
        return config;
      },
      // Parsing was not successful
      Err(e) => {
        println!("{}", Log::warning(e.to_string()));
        return default_values;
      }
    }
  }
}
