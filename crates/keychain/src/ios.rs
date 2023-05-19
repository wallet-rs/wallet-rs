// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Extremely inspired by the keyring implementation in keyring-rs:
//
// From: https://github.com/hwchen/keyring-rs/blob/2ce7dc54d66b919a848ce410cbce045f1fb7acb7/src/iOS.rs

use crate::{KeychainError, KeychainImpl};
use security_framework::passwords::{
    delete_generic_password, get_generic_password, set_generic_password,
};
use std::{
    cell::Cell,
    fmt::{Debug, Formatter},
    marker::PhantomData,
    sync::{Arc, Mutex},
};

pub trait KeychainBridge: Send + Sync + Debug {
    fn get_signing_key(&self, identifier: String) -> Result<String, KeychainError>;
}

/// Keychain struct for iOS.
pub struct IOSKeychain {
    // We use an internal mutex to ensure that we only have one thread accessing the keychain at a
    // time. This is because we use the same keychain for reads and writes, and we want to ensure
    // that we don't have a read and write happening at the same time.

    // From: https://github.com/sealvault/sealvault/blob/115701199aeae6976dfe78b709026f673d9f473a/core/src/encryption/keychains/ios_keychain.rs#L39-L43
    // It's a Mutex, instead of a RwLock because we only want access from one thread for reads as
    // well in order to zeroize the buffer returned from the keychain safely.
    internal: Arc<Mutex<IOSKeychainInternal>>,
}

/// Keychain implementation for iOS.
impl IOSKeychain {
    pub fn new() -> Self {
        let internal = Arc::new(Mutex::new(IOSKeychainInternal::new()));
        Self { internal }
    }
}

/// Default implementation for `IOSKeychain`.
impl Default for IOSKeychain {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug implementation for `IOSKeychain`.
impl Debug for IOSKeychain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IOSKeychain").finish()
    }
}

/// Keychain implementation for iOS.
impl KeychainImpl for IOSKeychain {
    fn name(&self) -> &str {
        "iOS"
    }

    fn get(&self, key: &str) -> Result<String, KeychainError> {
        let keychain = self.internal.lock()?;
        keychain.get(key)
    }

    fn set(&self, key: &str, value: &str) -> Result<(), KeychainError> {
        let keychain = self.internal.lock()?;
        keychain.set(key, value)
    }

    fn delete(&self, key: &str) -> Result<(), KeychainError> {
        let keychain = self.internal.lock()?;
        keychain.delete(key)
    }
}

/// Helper that we mark as not sync due to unsafe calls.
struct IOSKeychainInternal {
    // Hack to make `IOSKeychainInternal` not sync. A more elegant solution would be marking it is
    // `!Sync`, but that feature is unstable: https://github.com/rust-lang/rust/issues/68318
    _guard: PhantomData<Cell<()>>,
}

/// Internal keychain implementation for iOS.
impl IOSKeychainInternal {
    fn new() -> Self {
        Self { _guard: Default::default() }
    }

    fn get(&self, key: &str) -> Result<String, KeychainError> {
        let password_bytes = get_generic_password("wallet-rs", key)?;
        decode_password(password_bytes.to_vec())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), KeychainError> {
        set_generic_password("wallet-rs", key, value.as_bytes())?;
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), KeychainError> {
        delete_generic_password("wallet-rs", key)?;
        Ok(())
    }
}

/// Try to interpret a byte vector as a password string
pub fn decode_password(bytes: Vec<u8>) -> Result<String, KeychainError> {
    String::from_utf8(bytes).map_err(|_| KeychainError::Fatal { error: "Invalid UTF-8".into() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_keychain() {
        let keychain = Arc::new(IOSKeychain::new());
        // Allow clippy to clone the keychain because of the Arc.
        #[allow(clippy::redundant_clone)]
        let keychain2 = keychain.clone();

        // Test that we can set and get a value.
        keychain.set("test", "value").unwrap();
        assert_eq!(keychain.get("test").unwrap(), "value");

        // Test that we can delete a value.
        keychain.delete("test").unwrap();
        assert!(keychain.get("test").is_err());

        // Test that we can set and get a value.
        keychain2.set("test", "value").unwrap();
        assert_eq!(keychain2.get("test").unwrap(), "value");

        // Test that we can delete a value.
        keychain2.delete("test").unwrap();
        assert!(keychain2.get("test").is_err());
    }
}
