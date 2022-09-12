//! File hashing support.
mod value;

use std::io;
use std::path::Path;
use serde::{Serialize, Deserialize};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use tokio::io::{AsyncReadExt, BufReader};
use tokio::fs::File;

pub use value::DigestValue;

const MD5_SIZE: usize = 16;
const SHA1_SIZE: usize = 20;
const SHA256_SIZE: usize = 32;
const SHA512_SIZE: usize = 64;


/// A set of file hashes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiHash {
  #[serde(skip_serializing_if="Option::is_none")]
  pub md5: Option<DigestValue<MD5_SIZE>>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub sha1: Option<DigestValue<SHA1_SIZE>>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub sha256: Option<DigestValue<SHA256_SIZE>>,
  #[serde(skip_serializing_if="Option::is_none")]
  pub sha512: Option<DigestValue<SHA512_SIZE>>,
}

/// A set of digests for computing multiple hashes simultaneously.
pub struct MultiDigest {
  md5: Option<Md5>,
  sha1: Option<Sha1>,
  sha256: Option<Sha256>,
  sha512: Option<Sha512>,
}

fn maybe_update<D: Digest>(hash: &mut Option<D>, data: &[u8]) {
  if let Some(hash) = hash {
    hash.update(data);
  }
}

impl MultiDigest {
  /// Construct a new multi-digest with all hashes enabled.
  pub fn new() -> MultiDigest {
    MultiDigest {
      md5: Some(Md5::new()),
      sha1: Some(Sha1::new()),
      sha256: Some(Sha256::new()),
      sha512: Some(Sha512::new()),
    }
  }

  /// Update the hashes.
  pub fn update(&mut self, data: impl AsRef<[u8]>) {
    let data = data.as_ref();
    maybe_update(&mut self.md5, data);
    maybe_update(&mut self.sha1, data);
    maybe_update(&mut self.sha256, data);
    maybe_update(&mut self.sha512, data);
  }

  /// Finish this digest and return the hashes.
  pub fn finish(self) -> MultiHash {
    MultiHash {
      md5: self.md5.map(|h| h.finalize().into()),
      sha1: self.sha1.map(|h| h.finalize().into()),
      sha256: self.sha256.map(|h| h.finalize().into()),
      sha512: self.sha512.map(|h| h.finalize().into()),
    }
  }
}

pub async fn hash_file<P: AsRef<Path>>(path: P) -> io::Result<MultiHash> {
  let mut digest = MultiDigest::new();
  let file = File::open(path).await?;
  let mut read = BufReader::new(file);
  let mut buf = [0u8; 4096];

  loop {
    let n = read.read(&mut buf).await?;
    if n == 0 {
      break;
    }

    digest.update(&buf[..n]);
  }

  Ok(digest.finish())
}

#[tokio::test]
async fn test_hash_file() {
  let path = "Cargo.toml";
  let hashes = hash_file(path).await.expect("hash error");

  assert!(hashes.md5.is_some());
  assert!(hashes.sha1.is_some());
  assert!(hashes.sha256.is_some());
  assert!(hashes.sha512.is_some());

  let contents = tokio::fs::read(path).await.expect("io error");

  let mut md5 = Md5::new();
  md5.update(&contents);
  let md5: [u8; MD5_SIZE] = md5.finalize().into();
  assert_eq!(hashes.md5.unwrap().hash, md5);

  let mut sha1 = Sha1::new();
  sha1.update(&contents);
  let sha1: [u8; SHA1_SIZE] = sha1.finalize().into();
  assert_eq!(hashes.sha1.unwrap().hash, sha1);
}
