//! The ACF command-line interface.
//!
//! This module is intended to allow other program using AFC to also re-export AFC operations
//! from their own CLIs. The [AFC] struct defines the AFC command-line interface, except for
//! logging setup.  The AFC binary wraps this with additional options for verbosity, sets up
//! a log backend, and hands control off to the CLI.
use structopt::StructOpt;

/// Manage large data files through attached pointer files committed to VCS.
#[derive(StructOpt, Debug)]
#[structopt(name="astral-filing-cabinet")]
pub struct AFC {
}
