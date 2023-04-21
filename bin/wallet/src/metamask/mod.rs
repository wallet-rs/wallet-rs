use clap::Parser;

/// Start the node
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub async fn run(&self) -> eyre::Result<()> {
        Ok(())
    }
}
