// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use wallet_metamask::vault::extract_vault_from_file;

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
            mnemonic: "harvest afraid useful nose electric swift various man boil diagram confirm ahead",
            passphrase: "JooXegoodowu8mohf2ietah5kohgah5",
        },
        Fixture {
            path: "chromium-94.0.4606.81_4.17/000003.log",
            mnemonic: "very follow angry proof column rail smile intact broom chicken lens earth",
            passphrase: "aePaf7aequukoo6lahraitheemu6pein",
        },
        Fixture {
            path: "chromium-90-0.4430.72_2.14.1/Local Storage/leveldb/000003.log",
            mnemonic: "speed accuse odor ordinary exercise truly outer mask arrest life sibling height",
            passphrase: "",
        },
    ];

    /// Tests implemented from: https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.test.js
    #[test]
    fn encrypts_and_decrypts() {
        for f in FIXTURES.iter() {
            println!("decrypts {} {} {}", f.path, f.mnemonic, f.passphrase);
            let a = extract_vault_from_file(PathBuf::from("tests/fixtures").join(f.path));
            println!("{:?}", a);
            // decrypt_vault(a)
            // read file contents
            // call extractVaultFromFile function
            // call decryptVault function
            // assert result
        }
    }
}
