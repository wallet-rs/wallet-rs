// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Main entry point for the wallet cli.
/// Structue of the CLI is extremely influenced from reth.
/// https://github.com/paradigmxyz/reth/tree/main/bin/reth
use crate::metamask;
use clap::{ArgAction, Args, Parser, Subcommand};
use tracing::{metadata::LevelFilter, Level};
use tracing_subscriber::{filter::Directive, EnvFilter};

/// Parse CLI options, set up logging and run the chosen command.
pub async fn run() -> eyre::Result<()> {
    // Parse CLI options
    let opt = Cli::parse();

    // Set up logging based on the verbosity level
    let filter =
        EnvFilter::builder().with_default_directive(opt.verbosity.directive()).from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Run the chosen command
    match opt.command {
        Commands::Metamask(m) => m.run().await,
    }
}

/// Commands to be executed
#[derive(Subcommand)]
pub enum Commands {
    /// Run the metamask command utilities
    #[command(name = "metamask")]
    Metamask(metamask::Command),
}

#[derive(Parser)]
#[command(author, version = "0.1", about = "wallet-rs-cli", long_about = None)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Commands,

    #[clap(flatten)]
    verbosity: Verbosity,
}

#[derive(Args)]
#[command(next_help_heading = "Display")]
struct Verbosity {
    /// Set the minimum log level.
    ///
    /// -v      Errors
    /// -vv     Warnings
    /// -vvv    Info
    /// -vvvv   Debug
    /// -vvvvv  Traces
    #[clap(short, long, action = ArgAction::Count, global = true, default_value_t = 3, verbatim_doc_comment, help_heading = "Display")]
    verbosity: u8,

    /// Silence all log output.
    #[clap(long, alias = "silent", short = 'q', global = true, help_heading = "Display")]
    quiet: bool,
}

impl Verbosity {
    /// Get the corresponding [Directive] for the given verbosity, or none if the verbosity
    /// corresponds to silent.
    fn directive(&self) -> Directive {
        if self.quiet {
            LevelFilter::OFF.into()
        } else {
            let level = match self.verbosity - 1 {
                0 => Level::ERROR,
                1 => Level::WARN,
                2 => Level::INFO,
                3 => Level::DEBUG,
                _ => Level::TRACE,
            };

            level.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn test_cli_parse() -> Result<()> {
        // Test that the Cli struct can be parsed from command line arguments
        let cli = Cli::parse_from(["wallet-rs-cli", "metamask"]);
        assert!(matches!(cli.command, Commands::Metamask(_)));
        Ok(())
    }

    #[test]
    fn test_verbosity() {
        // Test that the verbosity level is correctly parsed
        let verbosity = Verbosity { verbosity: 1, quiet: false };
        assert_eq!(verbosity.directive(), LevelFilter::ERROR.into());

        let verbosity = Verbosity { verbosity: 2, quiet: false };
        assert_eq!(verbosity.directive(), LevelFilter::WARN.into());

        let verbosity = Verbosity { verbosity: 3, quiet: false };
        assert_eq!(verbosity.directive(), LevelFilter::INFO.into());

        let verbosity = Verbosity { verbosity: 4, quiet: false };
        assert_eq!(verbosity.directive(), LevelFilter::DEBUG.into());

        let verbosity = Verbosity { verbosity: 5, quiet: false };
        assert_eq!(verbosity.directive(), LevelFilter::TRACE.into());

        let verbosity = Verbosity { verbosity: 1, quiet: true };
        assert_eq!(verbosity.directive(), LevelFilter::OFF.into());
    }
}
