use happylog::structopt::LogOpts;
use structopt::StructOpt;
use anyhow::Result;

use astral_filing_cabinet::cli::AFC;

// Wrapper class that sets up logging.
#[derive(StructOpt, Debug)]
struct AFCCLI {
  #[structopt(flatten)]
  afc: AFC,

  #[structopt(flatten)]
  logging: LogOpts,
}

fn main() -> Result<()> {
  let opts = AFCCLI::from_args();
  opts.logging.init()?;

  opts.afc.invoke()
}
