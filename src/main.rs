use happylog::clap::LogOpts;
use clap::Parser;
use anyhow::Result;

use astral_filing_cabinet::cli::AFC;

// Wrapper class that sets up logging.
#[derive(Parser, Debug)]
struct AFCCLI {
  #[command(flatten)]
  afc: AFC,

  #[command(flatten)]
  logging: LogOpts,
}

fn main() -> Result<()> {
  let opts = AFCCLI::parse();
  opts.logging.init()?;

  opts.afc.invoke()
}
