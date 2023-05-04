// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(target_os = "linux")]
use in_memory::InMemoryKeychain;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
use std::sync::Arc;
#[cfg(target_os = "ios")]
use wallet_keychain::ios::IOSKeychain;
#[cfg(target_os = "macos")]
use wallet_keychain::macos::MacOSKeychain;
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
use wallet_keychain::KeychainImpl;

pub fn rust_greeting(to: String) -> String {
    format!("Hello World, {}!", to)
}

#[cfg(target_os = "ios")]
pub fn set_keychain(to: String) -> String {
    let keychain = Arc::new(IOSKeychain::new());
    #[allow(clippy::redundant_clone)]
    let _keychain2 = keychain.clone();

    // Test that we can set and get a value.
    _keychain2.set("test", &to).unwrap();

    format!("Hello World, {}!", to)
}

#[cfg(target_os = "linux")]
pub fn set_keychain(to: String) -> String {
    let keychain = Arc::new(InMemoryKeychain::new());
    #[allow(clippy::redundant_clone)]
    let _keychain2 = keychain.clone();

    // Test that we can set and get a value.
    _keychain2.set("test", &to).unwrap();

    format!("Hello World, {}!", to)
}

#[cfg(target_os = "macos")]
pub fn set_keychain(to: String) -> String {
    let keychain = Arc::new(MacOSKeychain::new());
    #[allow(clippy::redundant_clone)]
    let _keychain2 = keychain.clone();

    // Test that we can set and get a value.
    _keychain2.set("test", &to).unwrap();

    format!("Hello World, {}!", to)
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
