pub mod panic {
  use std::fs::File;
  use std::io::Write;
  use std::panic;
  use std::process;
  use backtrace::Backtrace;

  pub fn set_panic() {
    panic::set_hook(Box::new(|panic_info| {
      eprintln!("\x1b[31mAn unexpected error occurred!\x1b[0m\n");
      // Extract the error message
      let error_message = if let Some(info) = panic_info.payload().downcast_ref::<&str>() {
        info.to_string()
      } else if let Some(info) = panic_info.payload().downcast_ref::<String>() {
        info.clone()
      } else {
        "Unknown".to_string()
      };

      // Get backtrace
      let backtrace = Backtrace::new();

      // Get version of the binary
      let version = "1.0.0-beta22".to_string();

      // Create panic info string
      let panic_info_str = format!(
        "Error Message: {}\n\nBacktrace:\n{:?}\n\nBinary Version: {}",
        error_message, backtrace, version
      );

      // Write panic info to a file in the current working directory
      if let Ok(mut file) = File::create("panic_info.txt") {
        if let Err(err) = write!(file, "{}", panic_info_str) {
          eprintln!("Failed to write panic info to file: {}", err);
        }
      } else {
        eprintln!("Failed to create panic info file");
      }

      // Print formatted error message
      eprintln!("Please email us at square-db@outlook.com or submit an issue at https://github.com/square-db/square-db.\n");
      eprintln!("We do not collect information from your environment, so we rely on you to inform us. \nThank you.\n");

      process::exit(1);
    }));
  }
}