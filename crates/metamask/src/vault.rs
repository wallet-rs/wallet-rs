// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use base64::{engine::general_purpose, Engine as _};
/// Code from: https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs::File, io::Read, path::Path};

use crate::password::{decrypt, key_from_password};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vault {
    data: String,
    iv: String,
    salt: String,
}

pub fn extract_vault_from_file<P: AsRef<Path>>(path: P) -> Result<Vault, Box<dyn Error>> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let data = String::from_utf8_lossy(&data);

    extract_vault_from_string(&data)
}

pub fn extract_vault_from_string(data: &str) -> Result<Vault, Box<dyn Error>> {
    // Attempt 1: Try to parse as a JSON object
    if let Ok(vault) = serde_json::from_str::<Vault>(data) {
        return Ok(vault);
    }

    // Attempt 2: pre-v3 cleartext
    let matches = regex::Regex::new(r#"\{"wallet-seed":"([^"}]*)""#)?.find(data);
    if let Some(m) = matches {
        println!("Found pre-v3 vault");
        let mnemonic = (m.as_str().replace(r#"\""#, r#"""#) + "}").replace('\n', "");
        println!("{:?}", mnemonic);
        let vault_body: Value = serde_json::from_str(mnemonic.as_str()).unwrap();
        return Ok(Vault {
            data: vault_body["wallet-seed"].to_string(),
            iv: "".to_string(),
            salt: "".to_string(),
        });
    }

    // Attempt 3: chromium 000003.log file on linux
    let re = regex::Regex::new(r#""KeyringController":\{"vault":"\{[^{}]*}""#).unwrap();
    if let Some(capture) = re.captures(data) {
        println!("Found chromium vault");
        let vault_body_data = &capture[0][29..].replace(r#"\""#, r#"""#);
        let mut vault_body_data = vault_body_data.chars();
        vault_body_data.next();
        vault_body_data.next_back();
        let vault_body: Value = serde_json::from_str(vault_body_data.as_str()).unwrap();
        return Ok(Vault {
            data: vault_body["data"].to_string(),
            iv: vault_body["iv"].to_string(),
            salt: vault_body["salt"].to_string(),
        });
    }
    Err("Something went wrong".into())
}

pub fn decrypt_vault(vault: &Vault, password: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    // Define a regular expression that matches a BIP39 mnemonic phrase.
    let re = regex::Regex::new(r"^(?:\w{3,}\s+){11,}\w{3,}$").unwrap();

    // Check if the input string matches the regular expression.
    if re.is_match(&vault.data) {
        // If it matches, return the string.
        return Ok(vault.data.as_bytes().to_vec());
    }

    fn remove_first_last_three_chars(s: &str) -> &str {
        if s.len() <= 6 {
            return "";
        }
        &s[3..s.len() - 3]
    }

    // Otherwise, assume it's encrypted and decrypt it.
    println!("{:?}", remove_first_last_three_chars(&vault.data));
    let mut data = general_purpose::STANDARD
        .decode(remove_first_last_three_chars(&vault.data).as_bytes())
        .unwrap();
    println!("{:?}", remove_first_last_three_chars(&vault.salt));
    let salt = general_purpose::STANDARD
        .decode(remove_first_last_three_chars(&vault.salt).as_bytes())
        .unwrap();
    println!("{:?}", remove_first_last_three_chars(&vault.iv));
    let iv = general_purpose::STANDARD
        .decode(remove_first_last_three_chars(&vault.iv).as_bytes())
        .unwrap();
    let iv_slice = &iv[0..12];
    let iv_array: [u8; 12] = iv_slice.try_into().unwrap();
    println!("{:?}", iv_array);
    let key = key_from_password(password, Some(salt));
    let res = decrypt(&mut data, &key, Some(iv_array))?;
    Ok(res.to_vec())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chromium() {
        let data = r#"{"data":"8w0Wn8LaR3kMTp++Crr/JMCd6/xrfI1xWJsBgZXIdaKvPHCpjK/o1d6drEvQ7/ThtCynS5jP5F2T5esc0cin6E+2g3zcHRIpYp1Ut3Zn4Gw5Of8yxEk+Whq5eV2O8kbxfeurqTBx3b377e9Jd4N39QFF9kyE3cr8j6fETQvKjOC6irIGL0vI+TkUUylKISZ2OksbQJEooWPW3S1O8xdazL32j7dOnLbkrq1Xan0EIC7sg41oWUyMuS5eVopigxJ0ehueZsFlkvcBb+9zp6eMW5rw+CHC8KHXZdWGU45Ag85PaO5smtkOzb+WrQbufpQgsgKY23SsM8I1uTK6738/IHQ7kzFYImX0AJdF60xiUpihA/iUdWn6lr+kS4uyp7NhMLb4D5fHQi7pDb29TIDj1267rCD3w1N9M1nwWUjcG0gw5AMdf4bwYjpKOeQv2M5dGiX41+iQ9Rs5R6t3qZTNZpNu/czZaCUU8Bbr/je6Z7Milwl3b5NMfO7u2GID7aSG8s8RQ6/D5PjmtJN3a5BY6WLm1IzV","iv":"SCr2xR/hqI6qqJQese4E9Q==","salt":"HQnH0ArgfCWp86acfYN5Kr9wCWFKE3uw0fwUQafJHMY="}"#;
        println!("{:?}", data);
        // let vault = extract_vault_from_string(data);
        let vault_body: Value = serde_json::from_str(data).unwrap();
        println!("{:?}", vault_body);
    }
}
