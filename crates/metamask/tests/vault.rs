// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Test cases are from:
/// https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.test.js
use std::path::PathBuf;
use wallet_metamask::{
    types::StringOrBytes,
    vault::{decrypt_vault, extract_vault_from_file},
};

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    struct Fixture<'a> {
        path: &'a str,
        mnemonic: &'a str,
        passphrase: &'a str,
    }

    const FIXTURES: [Fixture; 4] = [
        Fixture {
            path: "chrome-windows-1/000005.ldb",
            mnemonic: "dolphin peanut amateur party differ tomorrow clean coconut when spatial hard trigger",
            passphrase: "t0b1m4ru",
        },
        Fixture {
            path: "chromium-108.0_5359.98_4.10.24.2/000003.log",
            mnemonic:
                "harvest afraid useful nose electric swift various man boil diagram confirm ahead",
            passphrase: "JooXegoodowu8mohf2ietah5kohgah5",
        },
        Fixture {
            path: "chromium-94.0.4606.81_4.17/000003.log",
            mnemonic: "very follow angry proof column rail smile intact broom chicken lens earth",
            passphrase: "aePaf7aequukoo6lahraitheemu6pein",
        },
        Fixture {
            path: "chromium-90-0.4430.72_2.14.1/Local Storage/leveldb/000003.log",
            mnemonic:
                "speed accuse odor ordinary exercise truly outer mask arrest life sibling height",
            passphrase: "",
        },
    ];

    /// Tests implemented from: https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.test.js
    #[test]
    fn encrypts_and_decrypts_fixtures() -> Result<()> {
        for f in FIXTURES.iter() {
            println!("Decrypting fixture: {}", f.path);

            // Extract the vault from the file
            let vault =
                extract_vault_from_file(PathBuf::from("tests/fixtures").join(f.path)).unwrap();
            println!("vault:");
            println!("{:?}", vault);

            // Decrypt the vault
            let s = decrypt_vault(&vault, f.passphrase).unwrap();
            println!("{:?}", s);

            // Check that the mnemonic matches
            let my_str = match s.data.mnemonic {
                StringOrBytes::String(s) => s.clone(),
                StringOrBytes::Bytes(_) => "".to_string(),
            };
            assert!(my_str == f.mnemonic);
        }
        Ok(())
    }
}
