//! The ACF command-line interface.
//!
//! This module is intended to allow other program using AFC to also re-export AFC operations
//! from their own CLIs. The [AFC] struct defines the AFC command-line interface, except for
//! logging setup.  The AFC binary wraps this with additional options for verbosity, sets up
//! a log backend, and hands control off to the CLI.
use structopt::StructOpt;
use anyhow::Result;
use tokio::runtime::Builder;

mod util;

/// Manage large data files through attached pointer files committed to VCS.
#[derive(StructOpt, Debug)]
#[structopt(name="astral-filing-cabinet")]
pub struct AFC {
  #[structopt(name="COMMAND", subcommand)]
  command: AFCCommand,
}

#[derive(StructOpt, Debug)]
enum AFCCommand {
  Util(util::UtilCommands),
}

impl AFC {
  /// Run the configured AFC command.
  ///
  /// This constructs a new [tokio::runtime::Runtime] and uses it to call
  /// [AFC::invoke_async()].
  pub fn invoke(&self) -> Result<()> {
    let runtime = Builder::new_multi_thread().enable_all().build()?;
    runtime.block_on(self.invoke_async())
  }

  /// Run the configured AFC command asynchronously.
  ///
  /// This method is used in code that is setting up its own
  /// [tokio::runtime::Runtime] and wants to run a task.
  pub async fn invoke_async(&self) -> Result<()> {
    match &self.command {
      AFCCommand::Util(cmd) => cmd.run().await
    }
  }
}
