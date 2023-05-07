// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// File is copied from sealvault's in_memory.rs
// From: https://github.com/sealvault/sealvault/blob/115701199aeae6976dfe78b709026f673d9f473a/core/src/encryption/keychains/in_memory_keychain.rs

use crate::{KeychainError, KeychainImpl};
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::{Arc, RwLock},
};

pub struct InMemoryKeychain {
    data: Arc<RwLock<HashMap<String, String>>>,
}

/// Keychain implementation for in memory use (for debugging and all purposes).
impl InMemoryKeychain {
    pub fn new() -> Self {
        InMemoryKeychain { data: Arc::new(RwLock::new(HashMap::new())) }
    }
}

/// Default implementation for `InMemoryKeychain`.
impl Default for InMemoryKeychain {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug implementation for `InMemoryKeychain`.
impl Debug for InMemoryKeychain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemoryKeychain").finish()
    }
}

/// Keychain implementation for in_memory.
impl KeychainImpl for InMemoryKeychain {
    fn name(&self) -> &str {
        "in_memory"
    }

    fn get(&self, key: &str) -> Result<String, KeychainError> {
        let d = self.data.read()?;
        let key = d.get(key).ok_or_else(|| KeychainError::NotFound { name: key.into() })?;
        Ok(key.to_string())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), KeychainError> {
        let mut d = self.data.write()?;
        d.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), KeychainError> {
        let mut d = self.data.write()?;
        let _ = d.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_keychain() {
        let keychain = Arc::new(InMemoryKeychain::new());
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
