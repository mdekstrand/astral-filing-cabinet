//! Pointers (references to artifacts) that are committed to git.

use relative_path::{RelativePath, RelativePathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFCPointer {
  pub path: RelativePathBuf,
}
