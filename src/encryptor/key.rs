use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCrypt256;
use magic_crypt::MagicCryptTrait;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std:: {
  process,
  env
};
use crate::log::log::*;
use crate::fm::fm:: {
  FileManager,
  FileManagerTrait
};

pub struct Key {
  mc: MagicCrypt256,
}

pub trait KeyTrait {
  fn new(password: &str) -> Key;
  fn decrypt_key(&self, encrypted_key: &str) -> Result<String,
  String>;
  fn encrypt_key(&self, key: &str) -> String;
  fn get_secret_key (&self) -> Result<String,
  String>;
  fn generate_key() -> String;
  fn check_pub_key(key: String) -> ();
  fn generate_valid_pub_key() -> String;
}

impl KeyTrait for Key {
  fn new(password: &str) -> Key {
    let mc = new_magic_crypt!(password.to_string(), 256);
    Key {
      mc
    }
  }

  fn generate_valid_pub_key() -> String {
    let mut rng = rand::thread_rng();

    let part1: String = rng.gen_range(1_000_000..999_999_999)
    .to_string();
    let part2: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(8)
    .map(char::from)
    .collect();
    let part3: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect();

    format!("SQU_{}:{}:{}", part1, part2, part3)
  }

  fn check_pub_key(key: String) ->() {
    if key.len() != 30 || !key.starts_with("SQU_") {
      println!("[{}] Signature error! Public key invalid! use --gen-key to generate one!", Log::error("ERR"));
      process::exit(1);
    }

    let parts: Vec<&str> = key.split(':').collect();
    if parts.len() != 3 || parts[0].len() != 13 || parts[1].len() != 8 || !parts[2].chars().all(char::is_alphanumeric) {
      println!("[{}] Signature error! Public key format invalid! use --gen-key to generate one!", Log::error("ERR"));
      process::exit(1);
    }
  }


  fn get_secret_key (&self) -> Result<String,
  String> {
    if let Ok(key) = FileManager::read_file_to_string("data/KMS/sq.key".to_string()) {
      match self.decrypt_key(&key) {
        Ok(result) => Ok(result),
        Err(e) => Err(e)
      }
    }
    else {
      println!("[{}] No Key was found new one is generated!", Log::info("INFO"));
      let key: String = Self::generate_key();
      if let Err(e) = FileManager::write_file_incremental("data/KMS/sq.key".to_string(), &self.encrypt_key(key.as_str())) {
        return Err(e.to_string());
      }
      return Ok(key);
    }

  }
  fn generate_key() -> String {
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(256)
    .map(char::from)
    .collect()
  }

  fn encrypt_key(&self, key: &str) -> String {
    self.mc.encrypt_str_to_base64(key)
  }

  fn decrypt_key(&self, encrypted_key: &str) -> Result<String,
  String> {
    match self.mc.decrypt_base64_to_string(encrypted_key) {
      Ok(result) => Ok(result),
      Err(_) => Err(String::from("Decryption error \n use --key --change to change it!"))
    }
  }
}

// function to get and handle KEYS
pub fn key() -> () {
  let pub_key = env::var("SQUARE_PUB_KEY").unwrap_or_else(|_| {
    println!("[{}] KMS : Public key is not set! set it in the .env using SQUARE_PUB_KEY=", Log::error("ERR"));
    process::exit(1);
  });
  Key::check_pub_key(pub_key.clone());
  let kms: Key = Key::new(&pub_key);
  if let Err(e) = kms.get_secret_key() {
    println!("[{}] KMS : KEY is invalid due to \n {} ", Log::error("ERR"), Log::error(e));
    process::exit(1);
  }
}