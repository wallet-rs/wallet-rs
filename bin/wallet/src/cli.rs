// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Main entry point for the wallet cli.
/// Structue of the CLI is extremely influenced from reth.
/// https://github.com/paradigmxyz/reth/tree/main/bin/reth
use crate::metamask;
use clap::{Parser, Subcommand};

/// Parse CLI options, set up logging and run the chosen command.
pub async fn run() -> eyre::Result<()> {
    let opt = Cli::parse();

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
#[command(author, version = "0.1", about = "Reth", long_about = None)]
struct Cli {
    /// The command to run
    #[clap(subcommand)]
    command: Commands,
}
