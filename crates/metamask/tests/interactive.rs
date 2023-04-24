// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Adds interactive test on the local machine
use wallet_metamask::interactive::{extract_all_vaults, get_password};
use wallet_metamask::vault::decrypt_vault;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    /// How to use the Vault Decryptor with the MetaMask Vault Data
    ///
    /// https://support.metamask.io/hc/en-us/articles/360018766351-How-to-use-the-Vault-Decryptor-with-the-MetaMask-Vault-Data
    #[allow(unused_attributes)]
    #[ignore = "This test can only be run on the local machine"]
    #[test]
    fn test_open_local() -> Result<()> {
        // Collect all vaults that are found locally
        let vaults = extract_all_vaults().unwrap();
        // Print the vault count
        println!("Found {} vaults", vaults.len());

        vaults.iter().for_each(|vault| {
            // Ask for password from user interactively
            let pwd = get_password().unwrap();

            // Attempt to decrypt the vault
            let res = decrypt_vault(vault, &pwd);

            println!("sucess! {}", res.is_ok());
        });
        Ok(())
    }
}
