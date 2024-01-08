use crate::fm::fm:: {
  FileManager,
  FileManagerTrait
};
use crate::table::table::Table;
use crate::encryptor::encryptor:: {
  Encryptor,
  EncryptorTrait
}; use crate::response::responses:: {
  Responses
};
use crate::response::process_responses::ProcessResponses;

pub struct Disk {
  encryptor: Encryptor,
}

pub trait DiskT {
  fn new() -> Self;
  fn write_table(&self, table: Table) -> Result<i32,
  Responses>;
  fn read_table(name: String) -> Result<Table,
  Responses>;
}

impl DiskT for Disk {
  fn new() -> Self {
    let binding = String::from("hello,world");
    let key: &str = binding.as_str();
    Disk {
      encryptor: {
        let encryptor_init: Encryptor = Encryptor::new(&key);
        encryptor_init
      }
    }
  }
  fn write_table(&self, table: Table) -> Result<i32,
  Responses> {
    let path_def: String = String::from("data/tables/0");
    let table_name: &String = &table.name;
    let path: String = format!("{}/{}", path_def, table_name);

    let encryptor: &Encryptor = &self.encryptor;
    let encrypted_table_data: String = encryptor.encrypt(encryptor.clone().instance, table.to_string().as_str());

    match FileManager::write_file_incremental(
      path,
      &encrypted_table_data
    ) {
      Ok(_) => Ok(0),
      Err(e) => Err(e)
    }
  }

  fn read_table(_name: String) -> Result<Table,
  Responses> {
    Err(Responses::Process(ProcessResponses::ReadingErr))
  }
}