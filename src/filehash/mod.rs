//! File hashing support.
mod value;


use serde::{Serialize, Deserialize};
use digest::Digest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

pub use value::DigestValue;

const MD5_SIZE: usize = 16;
const SHA1_SIZE: usize = 20;
const SHA256_SIZE: usize = 32;
const SHA512_SIZE: usize = 64;


/// A set of file hashes.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiHash {
  pub md5: Option<DigestValue<MD5_SIZE>>,
  pub sha1: Option<DigestValue<SHA1_SIZE>>,
  pub sha256: Option<DigestValue<SHA256_SIZE>>,
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
