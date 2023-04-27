// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[tokio::main]
/// Main entry point for the wallet cli.
///
/// From:
/// https://github.com/paradigmxyz/reth/blob/df6ff63806cc6d3aa168278514b8d854f771d4b6/bin/reth/src/main.rs
async fn main() {
    if let Err(err) = wallet_rs_cli::cli::run().await {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_main() {
        // Run the CLI with no arguments
        let output =
            Command::new("cargo").args(["run"]).output().expect("Failed to execute command");
        println!("output: {:?}", output);

        // Check that the CLI exited with an error (waiting status)
        assert!(!output.status.success());

        // Check that the CLI printed the help message
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("wallet-rs-cli"));
    }
}
