//! Manage large data files through pointer files stored in Git.
mod cache;
mod cli;
mod remote;
mod tree;
mod settings;

use cli::AFC;
use structopt::StructOpt;

// Wrapper class that sets up logging.
#[derive(StructOpt, Debug)]
struct AFCCLI {
  /// Increases logging verbosity.
  #[structopt(short="v", long="verbose", parse(from_occurrences))]
  verbose: i32,
  /// Silences informational status messages.
  #[structopt(short="q", long="quiet")]
  quiet: bool,

  #[structopt(flatten)]
  afc: AFC,
}

fn main() {
  let opts = AFCCLI::from_args();
}
