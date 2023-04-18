// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Code from: https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js
use crate::password::{decrypt, key_from_password};
use crate::types::{DecryptedVault, MnemoicData, StringOrBytes, Vault};
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use std::{collections::HashSet, error::Error, fs::File, io::Read, path::Path};

/// Extracts the vault from a file.
pub fn extract_vault_from_file<P: AsRef<Path>>(path: P) -> Result<Vault, Box<dyn Error>> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let data = String::from_utf8_lossy(&data);

    extract_vault_from_string(&data)
}

fn dedupe(arr: &[Value]) -> Vec<Value> {
    let mut result = Vec::new();
    for x in arr.iter() {
        let keys_x: HashSet<_> = x.as_object().unwrap().keys().collect();
        if !result.iter().any(|y: &Value| {
            let keys_y: HashSet<_> = y.as_object().unwrap().keys().collect();
            keys_x == keys_y && x == y
        }) {
            result.push(x.clone());
        }
    }
    result
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
    let matches = regex::Regex::new(r#"\{"wallet-seed":"([^"}]*)"#).unwrap().captures(data);
    if let Some(m) = matches {
        println!("Found pre-v3 vault");

        // Extract the mnemonic and parse it
        let mnemonic = m.get(1).map_or("", |m| m.as_str());
        let re = regex::Regex::new(r"\\n*").unwrap();
        let mnemonic = re.replace_all(mnemonic, "");

        // Extract the vault if it exists
        let vault_matches =
            regex::Regex::new(r#""wallet":("\{[ -~]*\\"version\\":2}")"#).unwrap().captures(data);
        let vault: Option<Vault> = vault_matches
            .and_then(|m| serde_json::from_str::<Vault>(m.get(1).map_or("", |m| m.as_str())).ok());

        // Return the vault if it exists, otherwise return the mnemonic
        println!("Your mnemonic is not encrypted");
        if let Some(vault) = vault {
            return Ok(vault);
        }
        return Ok(Vault { data: mnemonic.to_string(), iv: "".to_string(), salt: None });
    }

    // Attempt 3: chromium 000003.log file on linux
    let re = regex::Regex::new(r#""KeyringController":\{"vault":"\{[^{}]*}""#).unwrap();
    if let Some(capture) = re.captures(data) {
        println!("Found chromium vault");
        // TODO: Fix this hack #2
        let vault_body_data = &capture[0][29..].replace(r#"\""#, r#"""#);
        // TODO: Fix this hack #3
        let mut vault_body_data = vault_body_data.chars();
        vault_body_data.next();
        vault_body_data.next_back();
        let vault_body = serde_json::from_str::<Value>(vault_body_data.as_str());
        if vault_body.is_err() {
            return Err(Box::new(vault_body.err().unwrap()));
        }
        let vault_value = vault_body.unwrap();
        return Ok(Vault {
            data: vault_value["data"].to_string(),
            iv: vault_value["iv"].to_string(),
            salt: Some(vault_value["salt"].to_string()),
        });
    }

    // Attempt 4: chromium 000005.ldb on windows
    let match_regex = regex::Regex::new(r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu"#).unwrap();
    let capture_regex = regex::Regex::new(r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/u"#).unwrap();
    let iv_regex =
        regex::Regex::new(r#"/\\"iv.{1,4}[^A-Za-z0-9+\\/]{1,10}([A-Za-z0-9+\\/]{10,40}=*)/u"#)
            .unwrap();
    let data_regex = regex::Regex::new(r#"/\\"[^":,is]*\\":\\"([A-Za-z0-9+\\/]*=*)/u"#).unwrap();
    let salt_regex =
        regex::Regex::new(r#"/,\\"salt.{1,4}[^A-Za-z0-9+\\/]{1,10}([A-Za-z0-9+\\/]{10,100}=*)/u"#)
            .unwrap();

    let mut vaults: Vec<Value> = match_regex
        .find_iter(data)
        .map(|m| {
            print!("m: {:?}", m.as_str());
            capture_regex.find(m.as_str()).unwrap().as_str()
        })
        .map(|sm| {
            let mut d = None;
            let mut i = None;
            let mut s = None;
            for r in [&data_regex, &iv_regex, &salt_regex] {
                if let Some(caps) = r.captures(sm) {
                    match r {
                        _ if r.as_str() == data_regex.as_str() => d = Some(json!(caps[1])),
                        _ if r.as_str() == iv_regex.as_str() => i = Some(json!(caps[1])),
                        _ if r.as_str() == salt_regex.as_str() => s = Some(json!(caps[1])),
                        _ => unreachable!(),
                    }
                }
            }
            json!({
                "data": d.unwrap(),
                "iv": i.unwrap(),
                "salt": s.unwrap(),
            })
        })
        .collect();
    vaults = dedupe(&vaults);

    match vaults.len() {
        0 => {
            println!("Found no vaults!");
            Err("No vaults found".into())
        }
        1 => {
            println!("Found single vault! {:?}", vaults);
            Ok(Vault {
                data: vaults[0]["data"].clone().to_string(),
                iv: vaults[0]["iv"].clone().to_string(),
                salt: Some(vaults[0]["salt"].clone().to_string()),
            })
        }
        _ => {
            println!("Found multiple vaults! {:?}", vaults);
            Ok(Vault {
                data: vaults[0]["data"].clone().to_string(),
                iv: vaults[0]["iv"].clone().to_string(),
                salt: Some(vaults[0]["salt"].clone().to_string()),
            })
        }
    }
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

    // Decode the vault data.
    // This is a workaround for a bug in the extract_vault_from_string that has redundant quotes.
    fn decode(s: &str) -> String {
        if s.len() < 2 {
            return s.to_string();
        }

        let s = &s[1..s.len() - 1];
        println!("{}", s);
        s.to_string()
    }

    // Decode the vault data.
    let data = decode(&vault.data);
    let iv = decode(&vault.iv);
    let salt = &vault.salt.clone().map_or("".to_string(), |s| decode(&s));

    // Create a vault object.
    let mut cyphertext = Vault { data, iv, salt: Some(salt.to_string()) };

    // Attempt to decrypt the vault.
    let salt = general_purpose::STANDARD.decode(cyphertext.salt.clone().unwrap().as_bytes())?;
    let key = key_from_password(password, Some(&salt));
    let res = decrypt(password, &mut cyphertext, Some(&key))?;

    // Parse the decrypted vault data.
    let data = serde_json::from_str::<DecryptedVault>(&decode(&res));

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
}
