//use secrecy::{ExposeSecret, Secret};


pub struct Key {
  pub key: String,
}

pub trait KeyTrait {
  fn new() -> Key;
}

impl KeyTrait for Key {
  fn new() -> Key {
    Key {
      key: String::from("Hello, World")
    }
  }
}