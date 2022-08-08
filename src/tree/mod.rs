//! File tree operations.
use std::path::{PathBuf, Path};

use self::artifact::Artifact;

pub mod pointer;
pub mod artifact;

/// Representation of a working tree.
pub struct WorkTree {
  path: PathBuf,
}

impl WorkTree {
  /// Open a WorkTree at the specified location.
  pub fn open<P: AsRef<Path>>(path: P) -> WorkTree {
    let path = path.as_ref().to_owned();
    WorkTree { path }
  }

  pub fn scan_artifacts(&self) -> Vec<Artifact> {
    Vec::new()
  }
}
