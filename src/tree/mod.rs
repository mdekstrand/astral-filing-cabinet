//! File tree operations.
use std::io;
use std::path::{PathBuf, Path};

use futures::{TryStream, TryStreamExt};
use thiserror::Error;
use log::*;

pub mod pointer;
pub mod artifact;

use artifact::Artifact;
use relative_path::{RelativePathBuf, FromPathError};

use crate::util::walk::walk_directory;

/// An error that occured scanning the work tree.
#[derive(Error, Debug)]
pub enum ScanError {
  #[error("IO error occurred: {0}")]
  IOError(#[from] io::Error),
  #[error("failed to relativize path: {0}")]
  PathError(#[from] FromPathError),
}

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

  pub async fn scan_artifacts<'a>(&'a self) -> impl TryStream<Ok=Artifact, Error=ScanError> + 'a {
    let stream = walk_directory(self.root_path());
    let stream = stream.map_err(ScanError::IOError);
    stream.try_filter_map(move |de| async move {
      let fpath = de.path();
      trace!("scanning path {:?}", fpath);
      match fpath.extension() {
        Some(ext) if ext == "afc" => {
          let root = RelativePathBuf::from_path(self.root_path())?;
          let rp = RelativePathBuf::from_path(&fpath)?;
          let path = root.relative(rp);
          let art = Artifact::load_afc_pointer(self, &path).await?;
          Ok(Some(art))
        },
        _ => Ok(None)
      }
    })
    // futures::stream::iter(Vec::new())
  }
}
