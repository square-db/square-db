use dotenv::dotenv;
use crate::log::log::*;

pub struct Env;

pub trait EnvT{
  fn init() -> ();
}
impl EnvT for Env {
  fn init() -> (){
    match dotenv() {
      Ok(_) => println!("[{}] Loaded .env successfully", Log::info("INFO")),
      Err(_) => println!("[{}] Cannot load .env", Log::info("INFO"))
    }
  }
}