// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Code from: https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js
use crate::password::{decrypt, key_from_password};
use crate::{
    regex::{get_regex, RegexEnum},
    types::{DecryptedVault, MnemoicData, StringOrBytes, Vault},
};
use base64::{engine::general_purpose, Engine as _};
use itertools::Itertools;
use serde_json::Value;
use std::{error::Error, fs::File, io::Read, path::Path};
use tracing::{info, warn};

/// Extracts the vault from a file.
pub fn extract_vault_from_file<P: AsRef<Path>>(path: P) -> Result<Vault, Box<dyn Error>> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let data = String::from_utf8_lossy(&data);

    extract_vault_from_string(&data)
}

/// Splits a string with JSON objects into a vector of JSON objects.
fn split_json(s: &str) -> Vec<Value> {
    s.split(r#"}},"#)
        .flat_map(|s| {
            serde_json::from_str::<Value>(&format!("{}{}", s, r#"}}"#))
                .or_else(|_| serde_json::from_str::<Value>(s))
        })
        .collect()
}

/// Returns the result of decrypting the vault.
fn decrypt_vault_result(res: &str) -> Result<DecryptedVault, Box<dyn Error>> {
    // Parse the decrypted vault data.
    let data = serde_json::from_str::<DecryptedVault>(res);

    // If the data is a mnemonic, return it. If it is a bytes, convert it to a string and return it.
    if let Ok(vault) = data {
        match vault.data.mnemonic {
            StringOrBytes::String(s) => {
                let data = MnemoicData {
                    mnemonic: StringOrBytes::String(s),
                    number_of_accounts: vault.data.number_of_accounts,
                    hd_path: vault.data.hd_path,
                };
                let vault = DecryptedVault { r#type: vault.r#type, data };
                return Ok(vault);
            }
            StringOrBytes::Bytes(b) => {
                let data = MnemoicData {
                    mnemonic: StringOrBytes::String(std::str::from_utf8(&b).unwrap().to_string()),
                    number_of_accounts: vault.data.number_of_accounts,
                    hd_path: vault.data.hd_path,
                };
                let vault = DecryptedVault { r#type: vault.r#type, data };
                return Ok(vault);
            }
        }
    }

    Err(Box::new(data.err().unwrap()))
}

/// Extracts the vault from a file contents.
///
/// From:
/// https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L22
pub fn extract_vault_from_string(data: &str) -> Result<Vault, Box<dyn Error>> {
    // Attempt 1:
    // Try to parse as a JSON object
    // This is the case for objects that have not been encrypted
    if let Ok(vault) = serde_json::from_str::<Vault>(data) {
        return Ok(vault);
    }

    // Attempt 2: pre-v3 cleartext
    // If this is a pre-v3 vault, it will be a JSON object with a single key
    // Warns that the vault is not encrypted
    let matches = regex::Regex::new(&get_regex(RegexEnum::WalletSeed)).unwrap().captures(data);
    if let Some(m) = matches {
        info!("Found pre-v3 vault");

        // Extract the mnemonic and parse it
        let mnemonic = m.get(1).map_or("", |m| m.as_str());
        let re = regex::Regex::new(r"\\n*").unwrap();
        let mnemonic = re.replace_all(mnemonic, "");

        // Extract the vault if it exists
        let vault_matches =
            regex::Regex::new(&get_regex(RegexEnum::WalletV2)).unwrap().captures(data);
        let vault: Option<Vault> = vault_matches
            .and_then(|m| serde_json::from_str::<Vault>(m.get(1).unwrap().as_str()).ok());

        // Return the vault if it exists, otherwise return the mnemonic
        warn!("Your mnemonic is not encrypted");
        if let Some(vault) = vault {
            return Ok(vault);
        }
        return Ok(Vault { data: mnemonic.to_string(), iv: "".to_string(), salt: None });
    }

    // Attempt 3: chromium 000003.log file on linux
    let matches = regex::Regex::new(&get_regex(RegexEnum::Keyring)).unwrap().captures(data);
    if let Some(m) = matches {
        info!("Found chromium vault");

        // Extract the vault
        // Extra replace is to remove the extra backslashes
        // Also remove the first and last character
        //
        // Ref: https://github.com/MetaMask/vault-decryptor/blob/6cebd223816c80c3d879024aa385cb91fb49de0b/app/lib.js#L53
        let vault_body_data = &m[0][29..].replace(r#"\""#, r#"""#);
        let vault_body_data = vault_body_data[1..vault_body_data.len() - 1].to_string();

        // Parse the vault as json value
        let vault_body = serde_json::from_str::<Value>(&vault_body_data);
        if vault_body.is_err() {
            return Err(Box::new(vault_body.err().unwrap()));
        }

        // Return the vault
        let vault_value = vault_body.unwrap();
        return Ok(Vault {
            data: vault_value["data"].to_string(),
            iv: vault_value["iv"].to_string(),
            salt: Some(vault_value["salt"].to_string()),
        });
    }

    // Attempt 4: chromium 000005.ldb on windows
    // Attempts to match globaly
    let match_regex = regex::Regex::new(&get_regex(RegexEnum::MatchRegex)).unwrap();
    let capture_regex = regex::Regex::new(&get_regex(RegexEnum::CaptureRegex)).unwrap();
    let iv_regex = regex::Regex::new(&get_regex(RegexEnum::IVRegex)).unwrap();
    let data_regex = regex::Regex::new(&get_regex(RegexEnum::DataRegex)).unwrap();
    let salt_regex = regex::Regex::new(&get_regex(RegexEnum::SaltRegex)).unwrap();

    // Iterate over all matches and extract vaults
    let matches = match_regex.find_iter(data);
    let col: Vec<Vault> = matches
        .filter_map(|m| {
            let catches = capture_regex.captures(m.as_str());
            if let Some(m) = catches {
                let a = m.get(1).map_or("", |m| m.as_str());
                let iv = iv_regex.captures(a);
                let data = data_regex.captures(a);
                let salt = salt_regex.captures(a);

                if let (Some(i), Some(d), Some(s)) = (iv, data, salt) {
                    // Return with redundant quotes added
                    Some(Vault {
                        data: format!("\"{}\"", d.get(1).unwrap().as_str()),
                        iv: format!("\"{}\"", i.get(1).unwrap().as_str()),
                        salt: Some(format!("\"{}\"", s.get(1).unwrap().as_str())),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unique_by(|v| (v.iv.clone(), v.data.clone(), v.salt.clone()))
        .collect();

    // Return the first vault
    if !col.is_empty() {
        return Ok(col[0].clone());
    }

    Err("Could not extract vault".into())
}

/// Attempts to decrypt a vault.
/// If the vault is not encrypted, it will return the vault data.
///
/// From:
/// https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L92
pub fn decrypt_vault(vault: &Vault, password: &str) -> Result<DecryptedVault, Box<dyn Error>> {
    // Define a regular expression that matches a BIP39 mnemonic phrase.
    let re = regex::Regex::new(r"^(?:\w{3,}\s+){11,}\w{3,}$").unwrap();

    // Return the vault data if it is not encrypted.
    if re.is_match(&vault.data) || vault.salt.is_none() {
        let str = StringOrBytes::String(vault.data.to_string());
        let data = MnemoicData { mnemonic: str, number_of_accounts: None, hd_path: None };
        let vault = DecryptedVault { r#type: None, data };
        return Ok(vault);
    }

    // Remove redundant quotes from the vault data.
    fn remove_redundant_quotes(s: &str) -> String {
        if s.len() < 2 {
            return s.to_string();
        }

        let s = &s[1..s.len() - 1];
        s.to_string()
    }

    // Decode the vault data.
    let data = remove_redundant_quotes(&vault.data);
    let iv = remove_redundant_quotes(&vault.iv);
    let salt = &vault.salt.clone().map_or("".to_string(), |s| remove_redundant_quotes(&s));

    // Create a vault object.
    let mut cyphertext = Vault { data, iv, salt: Some(salt.to_string()) };

    // Attempt to decrypt the vault.
    let salt = general_purpose::STANDARD.decode(cyphertext.salt.clone().unwrap().as_bytes())?;
    let key = key_from_password(password, Some(&salt));
    let res = decrypt(password, &mut cyphertext, Some(&key))?;

    // Attempt to decrypt the vault.
    let r = decrypt_vault_result(&remove_redundant_quotes(&res));
    if r.is_ok() {
        return r;
    }

    // Split the vault data into multiple json objects, and attempt to decrypt each one.
    let json_vec = split_json(&remove_redundant_quotes(&res));
    for json_obj in json_vec {
        let res = decrypt_vault_result(&json_obj.to_string());
        if res.is_ok() {
            return res;
        }
    }

    Err("Could not decrypt vault".into())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_chromium() -> Result<()> {
        let data = r#"{"data":"8w0Wn8LaR3kMTp++Crr/JMCd6/xrfI1xWJsBgZXIdaKvPHCpjK/o1d6drEvQ7/ThtCynS5jP5F2T5esc0cin6E+2g3zcHRIpYp1Ut3Zn4Gw5Of8yxEk+Whq5eV2O8kbxfeurqTBx3b377e9Jd4N39QFF9kyE3cr8j6fETQvKjOC6irIGL0vI+TkUUylKISZ2OksbQJEooWPW3S1O8xdazL32j7dOnLbkrq1Xan0EIC7sg41oWUyMuS5eVopigxJ0ehueZsFlkvcBb+9zp6eMW5rw+CHC8KHXZdWGU45Ag85PaO5smtkOzb+WrQbufpQgsgKY23SsM8I1uTK6738/IHQ7kzFYImX0AJdF60xiUpihA/iUdWn6lr+kS4uyp7NhMLb4D5fHQi7pDb29TIDj1267rCD3w1N9M1nwWUjcG0gw5AMdf4bwYjpKOeQv2M5dGiX41+iQ9Rs5R6t3qZTNZpNu/czZaCUU8Bbr/je6Z7Milwl3b5NMfO7u2GID7aSG8s8RQ6/D5PjmtJN3a5BY6WLm1IzV","iv":"SCr2xR/hqI6qqJQese4E9Q==","salt":"HQnH0ArgfCWp86acfYN5Kr9wCWFKE3uw0fwUQafJHMY="}"#;
        println!("{:?}", data);
        // let vault = extract_vault_from_string(data);
        let vault_body: Value = serde_json::from_str(data).unwrap();
        println!("{:?}", vault_body);
        Ok(())
    }

    #[test]
    fn split_json_multiple() -> Result<()> {
        let s = r#"{"name":"Alice","sed":{}},{"name":"Bob","sed":{}},{"name":"Charlie","sed":{}}"#;
        let json_vec = split_json(s);
        assert_eq!(json_vec.len(), 3);
        let data = r#"{"type":"HD Key Tree","data":{"mnemonic":[100,111,108,112,104,105,110,32,112,101,97,110,117,116,32,97,109,97,116,101,117,114,32,112,97,114,116,121,32,100,105,102,102,101,114,32,116,111,109,111,114,114,111,119,32,99,108,101,97,110,32,99,111,99,111,110,117,116,32,119,104,101,110,32,115,112,97,116,105,97,108,32,104,97,114,100,32,116,114,105,103,103,101,114],"numberOfAccounts":1,"hdPath":"m/44'/60'/0'/0"}},{"type":"Ledger Hardware","data":{"hdPath":"m/44'/60'/0'","accounts":[],"accountDetails":{},"bridgeUrl":"https://metamask.github.io/eth-ledger-bridge-keyring","implementFullBIP44":false}}"#;
        let json_vec = split_json(data);
        assert_eq!(json_vec.len(), 2);
        Ok(())
    }
}
