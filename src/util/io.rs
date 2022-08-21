//! I/O utilities.
use std::io::Result;
use std::path::Path;

use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Read a file into a string.
pub async fn read_file_string<P: AsRef<Path>>(path: P) -> Result<String> {
  let mut file = File::open(path).await?;
  let mut content = String::new();
  file.read_to_string(&mut content).await?;
  Ok(content)
}
