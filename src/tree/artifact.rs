//! Representation for in-tree artifacts.

use relative_path::RelativePathBuf;

/// An artifact in the work tree.
pub struct Artifact {
  /// The path to this artifact within the work tree.
  path: RelativePathBuf,
  /// The path to this artifact's pointer file within the work tree.
  pointer: RelativePathBuf,
}
