use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Table {
  pub name: String,
  pub columns: HashMap<String,
  Vec<String>>,
  pub locked: String,
  pub locked_col: Vec<String>,
  pub defaults: HashMap<String,
  String>
}

impl Table {
  pub fn new() -> Table {
    Table {
      name: String::from("none"),
      columns: HashMap::new(),
      locked: String::from("false"),
      locked_col: vec!["id".to_string()],
      defaults: HashMap::new()
    }
  }
}

impl ToString for Table {
  fn to_string(&self) -> String {
    let columns_str: String = self
    .columns
    .iter()
    .map(|(name, types)| format!("{}-{}", name, types.join("|")))
    .collect::<Vec<String>>()
    .join(" , ");

    let locked_col_str = self.locked_col.join(",");
    let defaults_str: String = self
    .defaults
    .iter()
    .map(|(name, value)| format!("{}-{}", name, value))
    .collect::<Vec<String>>()
    .join(" , ");

    format!(
      "name: {};\ncolumns: {};\nlocked: {};\nlocked_col: {};\ndefaults: {};",
      self.name, columns_str, self.locked, locked_col_str, defaults_str
    )
  }
}