//! Implementation of file digest values.
use std::{fmt::Display, str::FromStr, convert::TryInto};

use generic_array::GenericArray;
use digest::OutputSizeUser;
use thiserror::Error;
use serde::{Serialize, Deserialize, de::{Visitor, Unexpected}};
use hex::{decode, encode, FromHexError};

/// Macro to create [From] implementations for generic arrays from crypto.
macro_rules! from_ga_impl {
    ($hash:ty, $size:literal) => {
      impl From<GenericArray<u8, <$hash as OutputSizeUser>::OutputSize>> for DigestValue<$size> {
        fn from(a: GenericArray<u8, <$hash as OutputSizeUser>::OutputSize>) -> DigestValue<$size> {
          let hash: [u8; $size] = a.into();
          DigestValue {
            hash
          }
        }
      }
    };
}

/// Error that occurred decoding a hash.
#[derive(Debug, Clone, Error)]
pub enum DigestDecodeError {
  #[error("expected {expected} bytes but found {found}")]
  InvalidLength {
    found: usize,
    expected: usize,
  },
  #[error("the hex-encoded string was invalid: {0}")]
  DecodeFailed(#[from] FromHexError),
}

/// A digest value with good serialization & I/O support.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct DigestValue<const SIZE: usize> {
  pub hash: [u8; SIZE]
}

impl <const N: usize> From<[u8; N]> for DigestValue<N> {
  fn from(hash: [u8; N]) -> Self {
    DigestValue { hash }
  }
}

impl <const N: usize> Into<[u8; N]> for DigestValue<N> {
  fn into(self) -> [u8; N] {
    self.hash
  }
}

impl <'a, const N: usize> Into<&'a [u8; N]> for &'a DigestValue<N> {
  fn into(self) -> &'a [u8; N] {
    &self.hash
  }
}

from_ga_impl!(md5::Md5, 16);
from_ga_impl!(sha1::Sha1, 20);
from_ga_impl!(sha2::Sha256, 32);
from_ga_impl!(sha2::Sha512, 64);

impl <const N: usize> Display for DigestValue<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = encode(self.hash);
    f.write_str(&s)
  }
}

impl <const N: usize> FromStr for DigestValue<N> {
  type Err = DigestDecodeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let bytes = decode(s)?;
    let hash = bytes.try_into().map_err(|v: Vec<u8>|  {
      DigestDecodeError::InvalidLength { found: v.len(), expected: N }
    })?;
    Ok(DigestValue {
      hash
    })
  }
}

impl <const N: usize> Serialize for DigestValue<N> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where S: serde::Serializer {
    if serializer.is_human_readable() {
      let s = encode(self.hash);
      serializer.serialize_str(&s)
    } else {
      serializer.serialize_bytes(&self.hash)
    }
  }
}

impl <'de, const N: usize> Deserialize<'de> for DigestValue<N> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where D: serde::Deserializer<'de> {
    deserializer.deserialize_str(HexDecodeVisitor::<N> {})
  }
}

struct HexDecodeVisitor<const N: usize> {}

impl <'de, const N: usize> Visitor<'de> for HexDecodeVisitor<N> {
  type Value = DigestValue<N>;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "hex string of length {}", N * 2)
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where E: serde::de::Error {
    DigestValue::from_str(v).map_err(|_| {
      E::invalid_value(Unexpected::Str(v), &self)
    })
  }

  fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
      where
          E: serde::de::Error, {
    let hash: [u8; N] = v.try_into().map_err(|_| {
      E::invalid_length(v.len(), &self)
    })?;
    Ok(DigestValue {
      hash
    })
  }
}

#[test]
fn test_hash_zero_encode() {
  let dv = DigestValue {
    hash: [0, 0, 0, 0]
  };
  let hex = dv.to_string();
  assert_eq!(&hex, "00000000");
}

#[test]
fn test_hash_zero_decode() {
  let str = "00000000";
  let dv = DigestValue::<4>::from_str(str);
  let dv = dv.expect("hash decode failed");
  assert_eq!(dv.hash, [0, 0, 0, 0]);
}
