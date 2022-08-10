//! Representation for in-tree artifacts.
use relative_path::{RelativePathBuf, RelativePath};

use super::pointer::AFCPointer;

/// An artifact in the work tree.
pub struct Artifact {
  /// The path to this artifact relative to the pointer file.
  path: RelativePathBuf,
  /// The path to this artifact's pointer file within the work tree.
  pointer_path: RelativePathBuf,
  /// The artifact pointer data
  pointer: AFCPointer,
}

impl Artifact {
  /// Get the path of this artifact, relative to the pointer file.
  pub fn path(&self) -> &RelativePath {
    self.path.as_relative_path()
  }

  /// Get the path of the pointer file.
  pub fn pointer_path(&self) -> &RelativePath {
    self.pointer_path.as_relative_path()
  }
}
