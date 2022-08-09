//! File tree operations.
use std::path::{PathBuf, Path};

use futures::Stream;

pub mod pointer;
pub mod artifact;

use artifact::Artifact;

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

  pub async fn scan_artifacts(&self) -> impl Stream<Item=Artifact> {
    futures::stream::iter(Vec::new())
  }
}
