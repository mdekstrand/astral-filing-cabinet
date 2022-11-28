//! Representation for in-tree artifacts.
use std::io;

use relative_path::{RelativePathBuf, RelativePath};
use serde::{Serialize, Deserialize};

use crate::filehash::MultiHash;

use super::{pointer::{AFCPointerFile}, WorkTree};

/// An artifact in the work tree.
pub struct Artifact {
  /// The path to this artifact within the work tree.
  tree_path: RelativePathBuf,
  /// The path to this artifact's pointer file within the work tree, if it has one.
  pointer_path: Option<RelativePathBuf>,
  /// The saved file metadata, if available.
  meta: Option<ArtifactMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactMeta {
  size: Option<usize>,
  #[serde(flatten)]
  hashes: MultiHash,
}

impl Artifact {
  pub async fn load_afc_pointer(tree: &WorkTree, path: &RelativePath) -> io::Result<Artifact> {
    let fp = path.to_path(tree.root_path());
    let ptr = AFCPointerFile::load(&fp).await?;
    let ptr = ptr.artifact;
    let dir = path.parent().map(RelativePath::to_owned);
    let dir = dir.unwrap_or_else(|| ".".into());
    let apath = dir.join(ptr.path());
    Ok(Artifact {
      tree_path: apath,
      pointer_path: Some(path.to_owned()),
      meta: Some(ptr.meta),
    })
  }

  /// Get the path of this artifact, relative to the pointer file.
  pub fn path(&self) -> &RelativePath {
    self.tree_path.as_relative_path()
  }

  /// Get the path of the pointer file.
  pub fn pointer_path(&self) -> Option<&RelativePath> {
    self.pointer_path.as_ref().map(|p| p.as_relative_path())
  }

  /// Get the artifact's metadata.
  pub fn meta(&self) -> Option<&ArtifactMeta> {
    self.meta.as_ref()
  }
}
