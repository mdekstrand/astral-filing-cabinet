//! Pointers (references to artifacts) that are committed to git.
use std::io;
use std::path::Path;

use log::*;
use relative_path::{RelativePath, RelativePathBuf};
use serde::{Serialize, Deserialize};

use crate::util::io::read_file_string;

use super::artifact::ArtifactMeta;

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

impl AFCPointerFile {
  /// Load an artifact from a pointer file.
  pub async fn load<P: AsRef<Path>>(path: P) -> io::Result<AFCPointerFile> {
    debug!("reading pointer file {:?}", path.as_ref());
    let content = read_file_string(path).await?;
    let obj = toml::from_str(&content)?;
    Ok(obj)
  }
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
  pub meta: ArtifactMeta,
}

impl AFCPointer {
  /// Get the path for this pointer.
  pub fn path(&self) -> &RelativePath {
    self.path.as_relative_path()
  }
}
