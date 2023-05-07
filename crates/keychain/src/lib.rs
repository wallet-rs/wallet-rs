// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(target_os = "macos")]
use crate::macos::MacOSKeychain;
#[cfg(target_os = "linux")]
use in_memory::InMemoryKeychain;
#[cfg(target_os = "ios")]
use ios::IOSKeychain;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use security_framework::base::Error;
#[cfg(target_os = "linux")]
pub mod in_memory;
#[cfg(target_os = "ios")]
pub mod ios;
#[cfg(target_os = "macos")]
pub mod macos;

/// Keychain is a trait that defines the interface for a keychain implementation
/// It is dependent on the OS, now we only support Linux and macOS
/// The keychain is used to store and retrieve secrets.
///
/// Extremely inspired by the keychain implementation in SealVault:
/// https://github.com/sealvault/sealvault/blob/115701199aeae6976dfe78b709026f673d9f473a/core/src/encryption/keychains/keychain.rs#L19-L64
pub trait KeychainImpl {
    /// Returns the name of the keychain implementation.
    fn name(&self) -> &str;

    /// Gets an item from the keychain.
    fn get(&self, key: &str) -> Result<String, KeychainError>;

    /// Sets an item in the keychain.
    fn set(&self, key: &str, value: &str) -> Result<(), KeychainError>;

    /// Delete an item from the keychain.
    fn delete(&self, key: &str) -> Result<(), KeychainError>;
}

#[derive(Debug)]
pub struct Keychain {
    #[cfg(target_os = "ios")]
    keychain: IOSKeychain,
    #[cfg(target_os = "macos")]
    keychain: MacOSKeychain,
    #[cfg(target_os = "linux")]
    keychain: InMemoryKeychain,
}

impl Keychain {
    #[cfg(target_os = "ios")]
    pub fn new() -> Self {
        let keychain = IOSKeychain::new();
        Self { keychain }
    }

    #[cfg(target_os = "linux")]
    pub fn new() -> Self {
        let keychain = InMemoryKeychain::new();
        Self { keychain }
    }

    #[cfg(target_os = "macos")]
    pub fn new() -> Self {
        let keychain = MacOSKeychain::new();
        Self { keychain }
    }

    pub fn get(&self, key: &str) -> Result<String, KeychainError> {
        self.keychain.get(key)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), KeychainError> {
        self.keychain.set(key, value)
    }

    pub fn delete(&self, key: &str) -> Result<(), KeychainError> {
        self.keychain.delete(key)
    }
}

impl Default for Keychain {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum KeychainError {
    /// A runtime invariant violation.
    #[error("Fatal Error: '{error}'")]
    Fatal { error: String },
    /// The keychain item was not found.
    #[error("Keychain item doesn't exist: '{name}'")]
    NotFound { name: String },
}

impl<T> From<std::sync::PoisonError<T>> for KeychainError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        KeychainError::Fatal { error: err.to_string() }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
impl From<Error> for KeychainError {
    fn from(err: Error) -> Self {
        KeychainError::Fatal { error: err.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keychain() {
        let keychain = Keychain::new();
        let key = "test_key";
        let value = "test_value";
        let _ = keychain.delete(key);
        let _ = keychain.set(key, value);
        let result = keychain.get(key);
        assert_eq!(result.unwrap(), value);
        let _ = keychain.delete(key);
        let result = keychain.get(key);
        assert!(result.is_err());
    }
}
