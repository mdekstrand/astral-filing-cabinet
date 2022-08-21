//! File tree operations.
use std::path::{PathBuf, Path};

use futures::{Stream, StreamExt};

pub mod pointer;
pub mod artifact;

use artifact::Artifact;

use crate::util::walk::walk_directory;

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

  /// Get the root path of this work tree.
  pub fn root_path(&self) -> &Path {
    self.path.as_path()
  }

  pub async fn scan_artifacts(&self) -> impl Stream<Item=Artifact> {
    futures::stream::iter(Vec::new())
  }
}
