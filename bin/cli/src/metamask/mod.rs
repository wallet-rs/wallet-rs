// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use eth_keystore::encrypt_key;
use ethers_signers::{coins_bip39::English, MnemonicBuilder};
use tracing::{debug, error, info};
use wallet_metamask::{
    interactive::{extract_all_vaults, get_password},
    vault::decrypt_vault,
};

/// Start the metamask command
#[derive(Debug, Parser)]
pub struct Command {
    /// Output the decrypted mnemonic to stdout
    #[arg(short, long)]
    output: bool,

    /// The path to the keystore file, if you want to export the encrypted keystore
    #[arg(short, long)]
    keystore: Option<String>,

    /// Flag to test running the command
    #[arg(short, long)]
    test: bool,
}

impl Command {
    pub async fn run(&self) -> eyre::Result<()> {
        // Get the vaults and the password
        let vaults = extract_all_vaults().unwrap();

        // Exit if this is a test run
        if self.test {
            info!("cargo test, exiting");
            return Ok(());
        }

        // Print the number of vaults
        info!("Found {} vaults", vaults.len());

        // Exit if there are no vaults
        if vaults.is_empty() {
            error!("No vaults found");
            return Ok(());
        }

        // Get the first vault and the password
        let vault = vaults[0].clone();
        let pwd = get_password().unwrap();

        // Attempt to decrypt the vault
        let res = decrypt_vault(&vault, &pwd);

        // Print the result
        if res.is_ok() {
            debug!("Decrypted vault");

            // Print the mnemonic
            if self.output {
                print!("{}", &res.unwrap().data.mnemonic);
                return Ok(());
            }

            if self.keystore.is_some() {
                // Get the mnemonic and index
                let index = 0u32;
                let phrase = &res.unwrap().data.mnemonic.to_string();

                // Build the wallet with the mnemonic
                let wallet = MnemonicBuilder::<English>::default()
                    .phrase(phrase.as_str())
                    .index(index)
                    .unwrap()
                    .build()
                    .unwrap();

                // Encrypt the wallet
                let pk = wallet.signer();
                let mut rng = rand::thread_rng();
                let _ =
                    encrypt_key(self.keystore.clone().unwrap(), &mut rng, pk.to_bytes(), pwd, None);
            }
        } else {
            error!("Failed to decrypt vault: {:?}", res);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[tokio::test]
    async fn test_metamask_run() {
        // Set up test input
        let command = Command { output: false, keystore: None, test: true };

        // Run the command
        let res = command.run().await;

        // Check that the command ran successfully
        assert!(res.is_ok());

        // Check that the logs contain the word "vault" (logs on found)
        assert!(logs_contain("cargo test, exiting"));
    }
}
