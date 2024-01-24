use magic_crypt::MagicCryptTrait;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCrypt256;

#[derive(Clone)]
pub struct Encryptor {
  pub instance: MagicCrypt256
}

pub trait EncryptorTrait {
  fn new(key : &str) -> Self;
  fn encrypt(&self, mcrypt: MagicCrypt256, txt: &str) -> String;
  fn decrypt(&self, mcrypt: MagicCrypt256, encrypted_string: &String) -> Result<String,Responses>;
}

impl EncryptorTrait for Encryptor {
  fn new(key : &str) -> Self {
    Encryptor {
      instance: {
        let mcrypt_init: MagicCrypt256 = new_magic_crypt!(key, 256);
        mcrypt_init
      }
    }
  }

  fn encrypt(&self, mcrypt: MagicCrypt256, txt: &str) -> String  {
    let encrypted_string : String = mcrypt.encrypt_str_to_base64(txt);
    encrypted_string
}

  fn decrypt(&self, mcrypt: MagicCrypt256, encrypted_string: &String) -> Result<String,Responses> {
    let decrypted_result = mcrypt.decrypt_base64_to_string(&encrypted_string);

    match decrypted_result {
      Ok(decrypt_text) => Ok(decrypt_text),
      Err(_) => Err(Responses::Process(ProcessResponses::DecryptionErr)),
    }

  }
}