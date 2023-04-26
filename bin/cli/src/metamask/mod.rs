// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use eth_keystore::encrypt_key;
use ethers_signers::{coins_bip39::English, MnemonicBuilder};
use tracing::{debug, info};
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
}

impl Command {
    pub async fn run(&self) -> eyre::Result<()> {
        // Get the vaults and the password
        let vaults = extract_all_vaults().unwrap();

        info!("Found {} vaults", vaults.len());

        let vault = vaults[0].clone();
        let pwd = get_password().unwrap();

        // Attempt to decrypt the vault
        let res = decrypt_vault(&vault, &pwd);

        // Print the result
        if res.is_ok() {
            debug!("Decrypted vault");

            // Print the mnemonic
            if self.output {
                println!("{}", &res.unwrap().data.mnemonic);
                return Ok(());
            }

            if self.keystore.is_some() {
                let index = 0u32;
                let phrase = &res.unwrap().data.mnemonic.to_string();

                let wallet = MnemonicBuilder::<English>::default()
                    .phrase(phrase.as_str())
                    .index(index)
                    .unwrap()
                    .build()
                    .unwrap();

                let pk = wallet.signer();
                let mut rng = rand::thread_rng();
                let _ =
                    encrypt_key(self.keystore.clone().unwrap(), &mut rng, pk.to_bytes(), pwd, None);
            }

            // let a = Wallet::<SigningKey>::new_keystore("", &mut rng, "randpsswd", None).unwrap();
        } else {
            info!("Failed to decrypt vault: {:?}", res);
        }
        Ok(())
    }
}
