// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use wallet_keychain::*;

pub fn rust_greeting(to: String) -> String {
    format!("Hello World, {}!", to)
}

pub fn set_keychain(key: String) -> String {
    let keychain = Keychain::new();
    let _ = keychain.set(&key, &key);
    format!("Set {}", key)
}

pub fn get_keychain(key: String) -> String {
    let keychain = Keychain::new();
    let _ = keychain.get(&key);
    format!("Get {}", key)
}

pub fn delete_keychain(key: String) -> String {
    let keychain = Keychain::new();
    let _ = keychain.delete(&key);
    format!("Delete {}", key)
}

uniffi_macros::include_scaffolding!("WalletCore");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_greeting() {
        assert_eq!(rust_greeting("Rust".to_string()), "Hello World, Rust!".to_string());
    }
}
