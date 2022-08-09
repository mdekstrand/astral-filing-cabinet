//! Test directory support code.
use std::fs::{DirBuilder, File, remove_dir_all};
use std::path::{PathBuf, Path};
use uuid::Uuid;

use log::*;

static BASE_DIR: &str = "target/test-dirs";
static TAR_DIR: &str = "tests/repos";

fn mkdirname(base: &str) -> String {
  let id = Uuid::new_v4();
  format!("{}-{}", base, id.as_hyphenated())
}

/// A test directory.
pub struct TestDir {
  path: PathBuf,
}

impl TestDir {
  pub fn empty() -> TestDir {
    let mut path = PathBuf::from(BASE_DIR);
    path.push(mkdirname("empty"));
    let td = TestDir { path };
    td.create();
    td
  }

  pub fn tarball(name: &str) -> TestDir {
    let mut path = PathBuf::from(BASE_DIR);
    path.push(mkdirname(name));
    let td = TestDir { path };
    td.create();

    let tpath = format!("{}/{}.tar", TAR_DIR, name);
    info!("extracting {}", tpath);
    let tarball = File::open(tpath).expect("error opening tarball");
    let mut arc = tar::Archive::new(tarball);
    arc.unpack(&td.path).expect("error extracting tarball");

    td
  }

  pub fn path(&self) -> &Path {
    self.path.as_path()
  }

  fn create(&self) {
    let mut db = DirBuilder::new();
    db.recursive(true);
    debug!("creating {:?}", self.path);
    db.create(&self.path).expect("mkdir failed");
  }
}

impl Drop for TestDir {
  fn drop(&mut self) {
    debug!("cleaning up {:?}", &self.path);
    remove_dir_all(&self.path).expect("directory cleanup failed");
  }
}
