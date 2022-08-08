use std::fs::DirBuilder;
use std::path::{PathBuf, Path};
use astral_filing_cabinet::tree::WorkTree;
use rstest::{fixture, rstest};

#[fixture]
fn empty_dir() -> PathBuf {
  let mut path = PathBuf::from("target");
  path.push("test-dirs");
  path.push("empty-dir");
  DirBuilder::new().recursive(true).create(&path).expect("mkdir failed");

  path
}

#[rstest]
fn test_empty_dir(empty_dir: PathBuf) {
  let tree = WorkTree::open(&empty_dir);
  let arts = tree.scan_artifacts();
  assert_eq!(arts.len(), 0);
}
