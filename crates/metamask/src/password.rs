// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Rust port of Metamask's password encryption/decryption logic.
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts

/// Inspired by:
/// https://github.com/fedimint/fedimint/blob/aa21c66582c17a68f19438366864652cba4bd590/crypto/aead/src/lib.rs#L25
/// https://docs.rs/ring/latest/ring/pbkdf2/index.html
use anyhow::{format_err, Result};
use base64::{engine::general_purpose, Engine as _};
use rand::{rngs::OsRng, thread_rng, Rng};
use ring::{
    aead,
    aead::{Aad, BoundKey, Nonce, AES_256_GCM, NONCE_LEN},
    digest, error, pbkdf2,
};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
pub struct Cyphertext {
    pub data: String,
    pub iv: String,
    pub salt: Option<String>,
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
pub fn encrypt(
    password: &str,
    data: &mut Vec<u8>,
    key: Option<&[u8]>,
    salt: Option<&str>,
) -> Result<String> {
    // Generate a salt if one is not provided.
    let salt = salt.map_or_else(|| None, |s| Some(general_purpose::STANDARD.encode(s)));
    let k = key_from_password(password, salt.as_ref().map(|s| s.as_bytes()));
    let key = key.unwrap_or(&k);

    // Generate the nonce (iv) from random bytes.
    let nonce = Nonce::assume_unique_for_key(OsRng.gen());
    let bytes_vec: Vec<u8> = nonce.as_ref().to_vec();

    // Construct a key from the provided bytes.
    let mut key: aead::SealingKey<OneNonceSequence> = make_key(&AES_256_GCM, key, nonce);

    // Encrypt the data.
    key.seal_in_place_append_tag(Aad::empty(), data)
        .map_err(|_| format_err!("Encryption failed due to unspecified aead error"))?;

    // Return the encrypted data.
    let text = Cyphertext {
        data: general_purpose::STANDARD.encode(data),
        iv: general_purpose::STANDARD.encode(bytes_vec),
        salt,
    };

    Ok(serde_json::to_string(&text).unwrap())
}

/// Decrypts a ciphertext using a key.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L103
pub fn decrypt(password: &str, ciphertext: &mut Cyphertext, key: Option<&[u8]>) -> Result<String> {
    // Decode the nonce and encrypted data.
    let mut data = general_purpose::STANDARD.decode(ciphertext.data.as_bytes())?;
    let nonce_bytes = general_purpose::STANDARD.decode(ciphertext.iv.as_bytes())?;
    let nonce_slice: [u8; NONCE_LEN] = *nonce_bytes.as_slice().array_chunks::<12>().next().unwrap();
    println!("nonce_bytes: {:?}", nonce_bytes);

    // Create a key from the password and salt
    let salt = ciphertext.salt.as_ref().map(|s| s.as_bytes());
    let k = key_from_password(password, salt);
    let key = key.unwrap_or(&k);

    // Construct a key from the provided bytes.
    let mut key: aead::OpeningKey<OneNonceSequence> =
        make_key(&AES_256_GCM, key, Nonce::assume_unique_for_key(nonce_slice));

    // Decrypt the data.
    key.open_in_place(Aad::empty(), data.as_mut_slice())
        .map_err(|_| format_err!("Decryption failed due to unspecified aead error"))?;

    // Return the decrypted data.
    // Omits the appended tag of the key algorithm.
    let d = std::str::from_utf8(&data[..data.len() - key.algorithm().tag_len()]);
    Ok(d.unwrap().to_string())
}

/// Derives a key from a password and random salt.
///
/// The key is derived using PBKDF2_HMAC_SHA256 with 10,000 iterations.
///
/// From:
/// https://github.com/MetaMask/browser-passworder/blob/a8574c40d1e42b2bc2c2b3d330b0ea50aa450017/src/index.ts#L214
pub fn key_from_password(password: &str, salt: Option<&[u8]>) -> [u8; 32] {
    let password = password.as_bytes();
    let random = generate_salt();
    let salt = salt.unwrap_or(&random);

    // Derive a key from the password using PBKDF2
    let mut pbkdf2_key: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(10_000).unwrap(),
        salt,
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

    #[test]
    fn encrypt_decrypt_test() {
        // determines the key, salt, and data
        let data = r#"
        {
            "cypher": "text"
        }"#;
        let data = serde_json::from_str::<serde_json::Value>(data).unwrap();
        let salt = "salt";
        let salt = general_purpose::STANDARD.decode(salt).unwrap();
        let key = key_from_password("password", Some(&salt));

        // encrypts the data
        let mut data = serde_json::to_vec(&data).unwrap();
        println!("data: {:?}", data);
        let ciphertext = encrypt("password", &mut data, Some(&key), Some("salt")).unwrap();
        println!("encrypted: {:?}", ciphertext);

        // decrypts the data
        let mut ciphertext = serde_json::from_str::<Cyphertext>(&ciphertext).unwrap();
        let res = decrypt("password", &mut ciphertext, Some(&key));
        println!("decrypted: {:?}", res);
    }

    #[test]
    fn encrypt_decrypt_test_real() {
        // taken from chromium-108.0_5359.98_4.10.24.2 example
        let data = r#"
        {    
            "data": "8w0Wn8LaR3kMTp++Crr/JMCd6/xrfI1xWJsBgZXIdaKvPHCpjK/o1d6drEvQ7/ThtCynS5jP5F2T5esc0cin6E+2g3zcHRIpYp1Ut3Zn4Gw5Of8yxEk+Whq5eV2O8kbxfeurqTBx3b377e9Jd4N39QFF9kyE3cr8j6fETQvKjOC6irIGL0vI+TkUUylKISZ2OksbQJEooWPW3S1O8xdazL32j7dOnLbkrq1Xan0EIC7sg41oWUyMuS5eVopigxJ0ehueZsFlkvcBb+9zp6eMW5rw+CHC8KHXZdWGU45Ag85PaO5smtkOzb+WrQbufpQgsgKY23SsM8I1uTK6738/IHQ7kzFYImX0AJdF60xiUpihA/iUdWn6lr+kS4uyp7NhMLb4D5fHQi7pDb29TIDj1267rCD3w1N9M1nwWUjcG0gw5AMdf4bwYjpKOeQv2M5dGiX41+iQ9Rs5R6t3qZTNZpNu/czZaCUU8Bbr/je6Z7Milwl3b5NMfO7u2GID7aSG8s8RQ6/D5PjmtJN3a5BY6WLm1IzV", 
            "iv": "SCr2xR/hqI6qqJQese4E9Q==", 
            "salt": "HQnH0ArgfCWp86acfYN5Kr9wCWFKE3uw0fwUQafJHMY=" 
        }
        "#;
        let ciphertext: Cyphertext = serde_json::from_str(data).unwrap();
        let salt = general_purpose::STANDARD.decode(ciphertext.salt.unwrap().as_bytes()).unwrap();
        let key = key_from_password("JooXegoodowu8mohf2ietah5kohgah5", Some(&salt));

        // decrypts the data
        let mut ciphertext = serde_json::from_str::<Cyphertext>(data).unwrap();
        let res = decrypt("JooXegoodowu8mohf2ietah5kohgah5", &mut ciphertext, Some(&key));
        println!("decrypted: {:?}", res);
    }

    #[test]
    fn key_from_password_test() {
        // salt is "salt"
        let salt = general_purpose::STANDARD.decode("salt".as_bytes()).unwrap();
        let key = key_from_password("password", Some(&salt));
        let answer = [
            "a6", "e9", "a9", "8f", "39", "01", "5c", "ba", "20", "58", "d4", "f4", "20", "fb",
            "f2", "b2", "e0", "ea", "e6", "73", "a7", "d4", "60", "b2", "1d", "e1", "9e", "ef",
            "f9", "4c", "f6", "db",
        ];

        // changes the array of u8 to an array of hex strings
        fn u8_array_to_hex_array(values: &[u8]) -> Vec<String> {
            values.iter().map(|&value| format!("{:02x}", value)).collect()
        }

        // iterates over the array to check if each element is equal
        answer.iter().zip(u8_array_to_hex_array(&key).iter()).for_each(|(a, b)| assert_eq!(a, b));
    }
}
