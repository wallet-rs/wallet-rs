// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use inquire::{Password, PasswordDisplayMode};
/// Adds interactive test on the local machine
use wallet_metamask::interactive::locate_metamask_extension;
use wallet_metamask::vault::{decrypt_vault, extract_vault_from_file};

#[cfg(test)]
#[allow(unused_attributes)]
#[ignore = "This test can only be run on the local machine"]
mod tests {
    use super::*;
    use anyhow::Result;

    fn get_password() -> Result<String> {
        let name = Password::new("Your metamask password:")
            .with_display_mode(PasswordDisplayMode::Masked)
            .prompt()
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(name)
    }

    /// How to use the Vault Decryptor with the MetaMask Vault Data
    /// https://support.metamask.io/hc/en-us/articles/360018766351-How-to-use-the-Vault-Decryptor-with-the-MetaMask-Vault-Data
    #[test]
    fn test_open_local() -> Result<()> {
        let a = locate_metamask_extension();
        if let Err(a) = a {
            let err = anyhow::anyhow!("Could not find MetaMask extension: {}", a);
            return Err(err);
        }
        let a = extract_vault_from_file(a.unwrap()).unwrap();

        // ask for password from user interactively
        let pwd = get_password().unwrap();

        let _ = decrypt_vault(&a, &pwd);
        Ok(())
    }
}
