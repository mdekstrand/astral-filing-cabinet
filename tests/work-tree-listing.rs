use astral_filing_cabinet::tree::WorkTree;
use futures::StreamExt;

mod common;
use common::TestDir;

#[tokio::test]
async fn test_empty_dir() {
  let dir = TestDir::empty();
  let tree = WorkTree::open(dir.path());
  let arts = tree.scan_artifacts().await;
  let arts: Vec<_> = arts.collect().await;
  assert_eq!(arts.len(), 0);
}

#[tokio::test]
async fn test_empty_repo() {
  let dir = TestDir::tarball("empty-git");
  let tree = WorkTree::open(dir.path());
  let arts = tree.scan_artifacts().await;
  let arts: Vec<_> = arts.collect().await;
  assert_eq!(arts.len(), 0);
}

#[tokio::test]
async fn test_single_artifact() {
  let dir = TestDir::tarball("single-artifact");
  let tree = WorkTree::open(dir.path());
  let arts = tree.scan_artifacts().await;
  let arts: Vec<_> = arts.collect().await;
  assert_eq!(arts.len(), 1);
  let art = &arts[0];
  assert_eq!(art.path().as_str(), "artifact.dat");
  assert_eq!(art.pointer_path().as_str(), "artifact.dat.afc");
}
