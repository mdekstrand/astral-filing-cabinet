//! Manage large data files through pointer files stored in Git.
mod util;
pub mod cache;
pub mod remote;
pub mod tree;
pub mod filehash;
pub mod settings;

#[cfg(feature="cli")]
pub mod cli;
