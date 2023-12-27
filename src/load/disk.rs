use crate::fm::fm:: {
  FileManager,
  FileManagerTrait
};
use crate::table::table::Table;

pub struct Disk;

pub trait DiskT {
  fn write_table(table : Table) -> Result<i32,i32>;
  fn read_table(name : String) -> Result<Table,i32>;
}

impl DiskT for Disk{
  fn write_table(table : Table) -> Result<i32,i32>{
    let path_def : String = String::from("data/tables/0");
    let table_name : &String =  &table.name;
    let path : String = format!("{}/{}" ,path_def,table_name);
    match FileManager::write_file_incremental(
      path,
      &table.to_string()
    ){
      Ok(_) => Ok(0),
      Err(e) => Err(e)
    }
  }
  
  fn read_table(_name : String) -> Result<Table,i32>{
    Err(604)
  }
}