pub mod state {
  //This script responsible for state container
  use crate::config::config::ConfigData;
  use lazy_static::lazy_static;
  use std::collections::HashMap;
  use std::sync::Mutex;

  //state container for saving data for lifecycels
  lazy_static! {
    pub static ref STATE_CONTAINER: Mutex<HashMap<String,
    ConfigData>> = Mutex::new(HashMap::new());
  }

  pub fn add_v_state(key: String, value: ConfigData) {
    let mut state = STATE_CONTAINER.lock().unwrap();
    state.insert(key, value);
  }
}