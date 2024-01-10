use std::collections::HashMap;
use crate::session::global_session::get_global_session;

pub struct SessionManager;
pub trait SessionManagerT {
  fn set(key: String, values: HashMap<String, String>) -> ();
  fn get(key: String) -> Option<HashMap<String,String>>;
}

impl SessionManagerT for SessionManager {
  // Function to set a key-value pair in the global session
  fn set(key: String, values: HashMap<String, String>) -> (){
    // Access the global session and obtain a lock
    let mut session = get_global_session().lock().unwrap();

    // Set the key-value pair in the session
    session.insert(key, values);
  }

  // Function to get the values associated with a key from the global session
  fn get(key: String) -> Option<HashMap<String,
  String>> {
    // Access the global session and obtain a lock
    let session = get_global_session().lock().unwrap();

    // Retrieve the values for the specified key
    session.get(&key).cloned()
  }
}