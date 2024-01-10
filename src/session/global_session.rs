use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref GLOBAL_SESSION: Mutex<HashMap<String,
  HashMap<String,
  String>>> = Mutex::new(HashMap::new());
}

// Function to access the global session
pub fn get_global_session() -> &'static Mutex<HashMap<String, HashMap<String, String>>> {
  &GLOBAL_SESSION
}