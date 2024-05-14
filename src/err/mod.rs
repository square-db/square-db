pub mod error {

  use thiserror::Error;
  
  //Future Use
  /*#[derive(Error, Debug, Clone)]
  pub enum EncryptionError {
    #[error("[ENC]: An Error occured while attempting to encrypt incoming data! Due to {0}")]
    EncryptionFailed(String),
    #[error("[ENC]: An Error occured while attempting to decrypt data! Due to {0}")]
    DecryptionFailed(String),
    #[error("[ENC]: An Error occured while attempting to create a encryption Key incoming data! Due to {0}")]
    KeyGenerationFailed(String)
  }*/
  
  #[derive(Error, Debug, Clone)]
  pub enum ServerError {
    #[error("[SERVER]: Required arguments were not been passed! {0} is empty.")]
    RequiredArgsError(String),
    #[error("[SERVER]: MAX_CONNECTIONS must be a valid positive integer.")]
    MaxConnectionsError,
    #[error("[SERVER]: WORKERS must be a valid positive integer greater than 0.")]
    WorkersNumberError
  }

}