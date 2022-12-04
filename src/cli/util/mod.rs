//! Utility commands - users generally won't need to use these.
use clap::Subcommand;
use anyhow::Result;

pub mod hash_file;

/// Internal utility commands.
///
/// These commands are primarily for testing and internal purposes.  Most users will
/// not need to use them.
#[derive(Subcommand, Debug, Clone)]
#[command(name="util")]
pub enum UtilCommands {
  HashFile(hash_file::HashFileCmd),
}

impl UtilCommands {
  pub async fn run(&self) -> Result<()> {
    match self {
      UtilCommands::HashFile(hf) => hf.run().await
    }
  }
}
