//! Pointers (references to artifacts) that are committed to git.

use relative_path::{RelativePath, RelativePathBuf};
use serde::{Serialize, Deserialize};

use crate::filehash::MultiHash;

/// Full AFC pointer file specification.
///
/// This struct realizes the schema for an AFC pointer file, which looks like this:
///
/// ```toml
/// [artifact]
/// path = "big-file.parquet"
/// md5 = "<...>"
/// sha1 = "<...>"
/// sha256 = "<...>"
/// sha512 = "<...>"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFCPointerFile {
  pub artifact: AFCPointer
}

impl From<AFCPointer> for AFCPointerFile {
  fn from(artifact: AFCPointer) -> Self {
    AFCPointerFile { artifact }
  }
}

impl Into<AFCPointer> for AFCPointerFile {
  fn into(self) -> AFCPointer {
    self.artifact
  }
}

/// Data for a native AFC pointer file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFCPointer {
  pub path: RelativePathBuf,
  #[serde(flatten)]
  pub hashes: MultiHash,
}

impl AFCPointer {
  /// Get the path for this pointer.
  pub fn path(&self) -> &RelativePath {
    self.path.as_relative_path()
  }
}
