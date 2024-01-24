pub mod create_response {
use std::collections::HashMap;
  pub fn create_response(
    st: &str,
    msg: &str,
    result: Option<&str>,
    description: Option<&str>,
  ) -> HashMap<String,
  String> {
    let mut response_map = HashMap::new();

    response_map.insert("StatusCode".to_string(), st.to_string());
    response_map.insert("msg".to_string(), msg.to_string());

    if let Some(res) = result {
      response_map.insert("result".to_string(), res.to_string());
    } else {
      response_map.insert("result".to_string(), "".to_string());
    }

    if let Some(desc) = description {
      response_map.insert("description".to_string(), desc.to_string());
    } else {
      response_map.insert("description".to_string(), "No description for that Error".to_string());
    }

    response_map
  }
}
pub mod response;