//! The `hash-file` command.
use std::path::PathBuf;
use anyhow::Result;
use log::*;

use structopt::StructOpt;

use crate::filehash::{hash_file, DigestValue};

/// Compute hashes for a file.
#[derive(StructOpt, Debug, Clone)]
#[structopt(name="hash-file")]
pub struct HashFileCmd {
  /// The file to hash.
  #[structopt(name="FILE")]
  file: PathBuf,
}

fn maybe_print_hash<const N: usize>(name: &str, hash: &Option<DigestValue<N>>) -> Result<()> {
  if let Some(hash) = hash {
    println!("{} = {}", name, hash);
  }
  Ok(())
}

impl HashFileCmd {
  pub async fn run(&self) -> Result<()> {
    info!("hashing file {:?}", &self.file);
    let hash = hash_file(&self.file).await?;
    maybe_print_hash("MD-5", &hash.md5)?;
    maybe_print_hash("SHA-1", &hash.sha1)?;
    maybe_print_hash("SHA-256", &hash.sha256)?;
    maybe_print_hash("SHA-512", &hash.sha512)?;
    Ok(())
  }
}
