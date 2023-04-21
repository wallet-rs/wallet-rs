// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use wallet_metamask::{
    interactive::{extract_all_vaults, get_password},
    vault::decrypt_vault,
};
/// Start the node
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub async fn run(&self) -> eyre::Result<()> {
        // Get the vaults and the password
        let vaults = extract_all_vaults().unwrap();
        let vault = vaults[0].clone();
        let pwd = get_password().unwrap();

        // Attempt to decrypt the vault
        let res = decrypt_vault(&vault, &pwd);

        // Print the result
        if res.is_ok() {
            println!("Decrypted vault");
        } else {
            println!("Failed to decrypt vault: {:?}", res);
        }
        Ok(())
    }
}
