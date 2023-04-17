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
    aead::{Aad, BoundKey, Nonce, AES_256_GCM, NONCE_LEN},
    digest, error, pbkdf2,
};
use std::{num::NonZeroU32, str};

pub static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

fn make_key<K: aead::BoundKey<OneNonceSequence>>(
    algorithm: &'static aead::Algorithm,
    key: &[u8],
    nonce: aead::Nonce,
) -> K {
    let key = aead::UnboundKey::new(algorithm, key).unwrap();
    let nonce_sequence = OneNonceSequence::new(nonce);
    K::new(key, nonce_sequence)
}

struct OneNonceSequence(Option<aead::Nonce>);

impl OneNonceSequence {
    /// Constructs the sequence allowing `advance()` to be called
    /// `allowed_invocations` times.
    fn new(nonce: aead::Nonce) -> Self {
        Self(Some(nonce))
    }
}

impl aead::NonceSequence for OneNonceSequence {
    fn advance(&mut self) -> Result<aead::Nonce, error::Unspecified> {
        self.0.take().ok_or(error::Unspecified)
    }
}

/// Constructs a key from a slice of bytes.
pub fn construct_key(key: &[u8]) -> aead::LessSafeKey {
    let key = aead::UnboundKey::new(&AES_256_GCM, key).map_err(|_| ()).unwrap();
    aead::LessSafeKey::new(key)
}
/// Encrypts a message using a key.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L32
pub fn encrypt(data: &mut Vec<u8>, key: &[u8], iv: Option<[u8; 12]>) -> Result<Vec<u8>> {
    // The target ciphertext for encryption.
    let mut ciphertext: Vec<u8> = vec![];

    // Generate the nonce from iv or random bytes.
    let nonce = Nonce::assume_unique_for_key(iv.unwrap_or(OsRng.gen()));
    let mut bytes_vec: Vec<u8> = nonce.as_ref().to_vec();

    // Construct a key from the provided bytes.
    let mut key: aead::SealingKey<OneNonceSequence> = make_key(&AES_256_GCM, key, nonce);

    // If an IV is provided, use it to encrypt the data.
    if iv.is_some() {
        key.seal_in_place_append_tag(Aad::empty(), data)
            .map_err(|_| format_err!("Encryption failed due to unspecified aead error"))?;
        ciphertext.append(data);
        return Ok(ciphertext);
    }

    key.seal_in_place_append_tag(Aad::empty(), data)
        .map_err(|_| format_err!("Encryption failed due to unspecified aead error"))?;

    ciphertext.append(&mut bytes_vec);
    ciphertext.append(data);
    Ok(ciphertext)
}

/// Decrypts a ciphertext using a key.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L103
pub fn decrypt<'c>(ciphertext: &'c mut [u8], key: &[u8], iv: Option<[u8; 12]>) -> Result<&'c [u8]> {
    // If an IV is provided, use it to decrypt the data.
    if let Some(iv) = iv {
        // Construct a key from the provided bytes.
        let mut key: aead::OpeningKey<OneNonceSequence> =
            make_key(&AES_256_GCM, key, Nonce::assume_unique_for_key(iv));
        println!("ciphertext: {:?}", ciphertext);
        println!("key: {:?}", key);
        println!("iv: {:?}", iv);
        key.open_in_place(Aad::empty(), ciphertext)
            .map_err(|_| format_err!("Decryption failed due to unspecified aead error with iv"))?;
        // Return the decrypted data.
        return Ok(&ciphertext[..ciphertext.len() - key.algorithm().tag_len()]);
    }

    // Check that the ciphertext is long enough to contain a nonce.
    if ciphertext.len() < NONCE_LEN {
        bail!("Ciphertext too short: {}", ciphertext.len());
    }

    // Split the ciphertext into the nonce and the encrypted data.
    let (nonce_bytes, encrypted_bytes) = ciphertext.split_at_mut(NONCE_LEN);

    // Construct a key from the provided bytes.
    let mut key: aead::OpeningKey<OneNonceSequence> = make_key(
        &AES_256_GCM,
        key,
        Nonce::assume_unique_for_key(nonce_bytes.try_into().expect("nonce size known")),
    );

    // Decrypt the data.
    key.open_in_place(Aad::empty(), encrypted_bytes)
        .map_err(|_| format_err!("Decryption failed due to unspecified aead error"))?;

    // Return the decrypted data.
    Ok(&encrypted_bytes[..encrypted_bytes.len() - key.algorithm().tag_len()])
}

pub fn u8_array_to_hex_array(values: &[u8]) -> Vec<String> {
    values.iter().map(|&value| format!("{:02x}", value)).collect()
}

/// Derives a key from a password and random salt.
///
/// The key is derived using PBKDF2_HMAC_SHA256 with 10,000 iterations.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L214
pub fn key_from_password(password: &str, salt: Option<Vec<u8>>) -> [u8; 32] {
    let password = password.as_bytes();
    let salt = salt.unwrap_or_else(generate_salt);

    // Derive a key from the password using PBKDF2
    let mut pbkdf2_key: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(10_000).unwrap(),
        &salt,
        password,
        &mut pbkdf2_key,
    );

    pbkdf2_key
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
    use base64::{engine::general_purpose, Engine as _};

    /// Tests implemented from: https://github.com/fedimint/fedimint/blob/aa21c66582c17a68f19438366864652cba4bd590/crypto/aead/src/lib.rs#L131
    #[test]
    fn encrypts_and_decrypts() {
        let password = "test123";
        let message = "hello world";

        let key = key_from_password(password, None);
        let mut cipher_text = encrypt(&mut message.as_bytes().to_vec(), &key, None).unwrap();
        let decrypted = decrypt(&mut cipher_text, &key, None).unwrap();

        assert_eq!(decrypted, message.as_bytes());
    }

    #[test]
    fn encrypts_and_decrypts_with_iv() {
        let password = "test123";
        let message = "hello world";
        let salt = generate_salt();
        let iv: [u8; 12] = OsRng.gen();

        let key = key_from_password(password, Some(salt));
        let mut cipher_text = encrypt(&mut message.as_bytes().to_vec(), &key, Some(iv)).unwrap();
        let decrypted = decrypt(&mut cipher_text, &key, Some(iv)).unwrap();

        assert_eq!(decrypted, message.as_bytes());
    }

    #[test]
    fn key_from_password_2_test() {
        let b = general_purpose::STANDARD.decode("salt".as_bytes()).unwrap();
        let a = key_from_password("password", Some(b));
        let answer = [
            "a6", "e9", "a9", "8f", "39", "01", "5c", "ba", "20", "58", "d4", "f4", "20", "fb",
            "f2", "b2", "e0", "ea", "e6", "73", "a7", "d4", "60", "b2", "1d", "e1", "9e", "ef",
            "f9", "4c", "f6", "db",
        ];
        // iterates over the array to check if each element is equal
        answer.iter().zip(u8_array_to_hex_array(&a).iter()).for_each(|(a, b)| assert_eq!(a, b));
    }
}
