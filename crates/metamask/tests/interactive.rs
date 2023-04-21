// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use inquire::{Password, PasswordDisplayMode};
/// Adds interactive test on the local machine
use wallet_metamask::interactive::locate_metamask_extension;
use wallet_metamask::vault::{decrypt_vault, extract_vault_from_file};

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result};
    use wallet_metamask::types::Vault;

    fn get_password() -> Result<String> {
        let name = Password::new("Your metamask password:")
            .with_display_mode(PasswordDisplayMode::Masked)
            .prompt()
            .map_err(|e| anyhow!(e))?;

        Ok(name)
    }

    /// How to use the Vault Decryptor with the MetaMask Vault Data
    ///
    /// https://support.metamask.io/hc/en-us/articles/360018766351-How-to-use-the-Vault-Decryptor-with-the-MetaMask-Vault-Data
    #[allow(unused_attributes)]
    #[ignore = "This test can only be run on the local machine"]
    #[test]
    fn test_open_local() -> Result<()> {
        // Attempt to locate the MetaMask extension
        let a = locate_metamask_extension();
        if let Err(a) = a {
            let err = anyhow!("Error while finding MetaMask extension: {}", a);
            return Err(err);
        }

        // Iterate over all vaults
        let vaults: Vec<Vault> = a
            .unwrap()
            .iter()
            .filter_map(|a| {
                println!("Attempting to decrypt vault: {:?}", a);

                // Attempt to extract the vault from the extension
                let vault = extract_vault_from_file(a);

                if let Ok(vault) = vault {
                    Some(vault)
                } else {
                    None
                }
            })
            .collect();

        println!("Found {} vaults: {:?}", vaults.len(), vaults);

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
