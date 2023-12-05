#[path = "../src/entry/mod.rs"]
mod entry;
use entry::entry::Entry;
use entry::entry::EntryTrait;

#[test]
fn test_handle_cmd(){
  /* Testing lowercase feautre*/
  //assert_eq!(Ok(100) , Entry::new("dev").handle_cmd("PiNg")); // passed
  
  /* Testing #[u] feautre*/
  //assert_eq!(Err(1000) , Entry::new("dev").handle_cmd("#[u]PING#[u]")); //passed
  
  /* Testing spamming #[u] */
  //assert_eq!(Err(1000) , Entry::new("dev").handle_cmd("#[u]PING#[u]#[u]")); //passed 
}