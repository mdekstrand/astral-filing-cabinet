//! Tree-walking utility code.
//!
//! This code implements async-friendly tree-walking.  It is somewhat like
//! [tokio::fs::read_dir], except recursive and only using a single blocking
//! task for the entire recursive walk.  It yields a stream.
use std::collections::LinkedList;
use std::path::Path;
use std::io;
use std::fs::{DirEntry, read_dir};

use log::*;

use futures::Stream;
use tokio::sync::mpsc::{unbounded_channel};
use tokio::task::spawn_blocking;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[cfg(test)]
use relative_path::RelativePathBuf;
#[cfg(test)]
use futures::StreamExt;

/// Recursively walk a directory.
///
/// This function recursively walks a directory, yielding each directory entry via a stream.
/// I/O errors encountered during traversal are reported as errors on the stream, and the
/// walk terminates after the first error it finds.  Channel errors cause this function's
/// background task to panic.
pub fn walk_directory<P: AsRef<Path>>(path: P) -> impl Stream<Item=io::Result<DirEntry>> {
  let (send, recv) = unbounded_channel();

  // get an owned copy of the path
  let path = path.as_ref().to_path_buf();

  let _jh = spawn_blocking(move || {
    let mut queue = LinkedList::new();
    queue.push_back(path);

    while let Some(path) = queue.pop_front() {
      let rdr = read_dir(&path);
      match rdr {
        Ok(rd) => {
          for der in rd {
            if let Ok(de) = &der {
              match de.file_type() {
                Ok(ft) if ft.is_dir() => {
                  queue.push_back(de.path())
                },
                Err(e) => {
                  // we got the dirent, but can't get the file type - error
                  error!("{:?}: cannot read file type: {:?}", de.path(), e);
                  send.send(Err(e)).expect("channel failed");
                  return;
                }
                _ => () // fall through to send to queue
              }
            } else {
              error!("{:?}: error reading directory entry: {:?}", path, der.as_ref().err().unwrap());
            }

            send.send(der).expect("channel failed");
          }
        },
        Err(e) => {
          error!("{:?}: cannot read directory: {:?}", path, e);
          send.send(Err(e)).expect("channel send error");
          return;  // we abandon on first error
        }
      }
    }
  });

  UnboundedReceiverStream::new(recv)
}


/// Test walking the source tree, just for basic functionality.
#[tokio::test]
async fn test_walk_source() {
  let mut stream = walk_directory("src");
  let mut files = Vec::new();

  while let Some(de) = stream.next().await {
    let de = de.expect("directory entry failure");
    let ft = de.file_type().expect("file type failure");
    if ft.is_file() {
      let path = de.path();
      let path = RelativePathBuf::from_path(path).expect("path error");
      files.push(path);
    }
  }

  assert!(files.len() > 10);
  assert!(files.contains(&RelativePathBuf::from("src/tree/walk.rs")));
}
