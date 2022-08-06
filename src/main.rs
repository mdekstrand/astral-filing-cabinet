use astral_filing_cabinet::cli::AFC;
use happylog::LogOpts;
use structopt::StructOpt;
use anyhow::Result;

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

  Ok(())
}
