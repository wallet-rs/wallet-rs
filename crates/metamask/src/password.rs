// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Rust port of Metamask's password encryption/decryption logic.
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts

/// Inspired by:
/// https://github.com/fedimint/fedimint/blob/aa21c66582c17a68f19438366864652cba4bd590/crypto/aead/src/lib.rs#L25
/// https://docs.rs/ring/latest/ring/pbkdf2/index.html
use anyhow::{bail, format_err, Result};
use rand::{rngs::OsRng, thread_rng, Rng};
use ring::{
    aead,
    aead::{Aad, Nonce, AES_256_GCM, NONCE_LEN},
    digest, pbkdf2,
    rand::{SecureRandom, SystemRandom},
};
use std::{num::NonZeroU32, str};

pub static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

/// Constructs a key from a slice of bytes.
pub fn construct_key(key: &[u8]) -> aead::LessSafeKey {
    let key = aead::UnboundKey::new(&AES_256_GCM, key).map_err(|_| ()).unwrap();
    aead::LessSafeKey::new(key)
}
/// Encrypts a message using a key.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L32
pub fn encrypt(data: &mut Vec<u8>, key: &[u8]) -> Result<Vec<u8>> {
    // Generate a random nonce.
    let rng = SystemRandom::new();
    let mut nonce = [0u8; NONCE_LEN];
    rng.fill(&mut nonce).map_err(|_| ()).unwrap();

    // Construct a key from the provided bytes.
    let key = construct_key(key);

    // Encrypt the data.
    let nonce = Nonce::assume_unique_for_key(OsRng.gen());
    let mut ciphertext: Vec<u8> = nonce.as_ref().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), data)
        .map_err(|_| format_err!("Encryption failed due to unspecified aead error"))?;

    ciphertext.append(data);
    Ok(ciphertext)
}

/// Decrypts a ciphertext using a key.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L103
pub fn decrypt<'c>(ciphertext: &'c mut [u8], key: &[u8]) -> Result<&'c [u8]> {
    // Check that the ciphertext is long enough to contain a nonce.
    if ciphertext.len() < NONCE_LEN {
        bail!("Ciphertext too short: {}", ciphertext.len());
    }
    // Split the ciphertext into the nonce and the encrypted data.
    let (nonce_bytes, encrypted_bytes) = ciphertext.split_at_mut(NONCE_LEN);

    // Construct a key from the provided bytes.
    let key = construct_key(key);

    // Decrypt the data.
    key.open_in_place(
        Nonce::assume_unique_for_key(nonce_bytes.try_into().expect("nonce size known")),
        Aad::empty(),
        encrypted_bytes,
    )
    .map_err(|_| format_err!("Decryption failed due to unspecified aead error"))?;

    // Return the decrypted data.
    Ok(&encrypted_bytes[..encrypted_bytes.len() - key.algorithm().tag_len()])
}

/// Derives a key from a password and random salt.
///
/// The key is derived using PBKDF2_HMAC_SHA256 with 10,000 iterations.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L214
pub fn key_from_password(password: &str) -> [u8; 32] {
    let salt = generate_salt();
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        PBKDF2_ALG,
        NonZeroU32::new(10_000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut to_store,
    );
    to_store
}

/// Generates a random salt.
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L299
pub fn generate_salt() -> Vec<u8> {
    let mut rng = thread_rng();
    let mut salt = vec![0; 32];
    rng.fill(&mut salt[..]);
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests implemented from: https://github.com/fedimint/fedimint/blob/aa21c66582c17a68f19438366864652cba4bd590/crypto/aead/src/lib.rs#L131
    #[test]
    fn encrypts_and_decrypts() {
        let password = "test123";
        let message = "hello world";

        let key = key_from_password(password);
        let mut cipher_text = encrypt(&mut message.as_bytes().to_vec(), &key).unwrap();
        let decrypted = decrypt(&mut cipher_text, &key).unwrap();

        assert_eq!(decrypted, message.as_bytes());
    }
}
