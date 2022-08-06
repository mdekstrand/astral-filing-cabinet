//! Manage large data files through pointer files stored in Git.
pub mod cache;
pub mod remote;
pub mod tree;
pub mod settings;

#[cfg(feature="cli")]
pub mod cli;
