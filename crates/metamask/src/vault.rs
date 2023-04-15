// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Code from: https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs::File, io::Read, path::Path};
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
    let vault_body: Vault;
    if let Ok(vault) = serde_json::from_str(data) {
        return Ok(vault);
    }

    // Attempt 2: pre-v3 cleartext
    let matches = regex::Regex::new(r#"\{"wallet-seed":"([^"}]*)""#)?.find(data);
    if let Some(m) = matches {
        println!("Found pre-v3 vault");
        let mnemonic = m.as_str().replace(r#"\\n"#, "");
        let vault_matches =
            regex::Regex::new(r#""wallet":("\{[ -~]*\\"version\\":2}")"#)?.find(data);
        let vault: String = if let Some(m) = vault_matches {
            serde_json::from_str(m.as_str())?
        } else {
            return Err("Could not find vault".into());
        };
        return Ok(Vault { data: vault, iv: "".to_string(), salt: "".to_string() });
    }

    // Attempt 3: chromium 000003.log file on linux
    let re = regex::Regex::new(r#""KeyringController":\{"vault":"\{[^{}]*}""#).unwrap();
    if let Some(capture) = re.captures(data) {
        println!("Found chromium vault");
        let vault_body_data = &capture[0][29..];
        let mut vault_body_data = vault_body_data.chars();
        vault_body_data.next();
        vault_body_data.next_back();
        println!("{}", vault_body_data.as_str());
        let vault_body: Value = serde_json::from_str(vault_body_data.as_str()).unwrap();
        println!("{}", vault_body_data.as_str());
        let vault_body: Value = serde_json::from_str(vault_body_data.as_str()).unwrap();
        // let vault_body = serde_json::from_str::<String>(vault_body_data)?;
        // println!("{:?}", serde_json::from_str::<Vault>(vault_body_data));
        let vault_body: Value = serde_json::from_str(data).unwrap();
        println!("here2");
        println!("{:?}", vault_body);
        return Ok(Vault {
            data: vault_body["data"].to_string(),
            iv: vault_body["iv"].to_string(),
            salt: vault_body["salt"].to_string(),
        });
    }
    Err("Something went wrong".into())
}

pub fn decrypt_vault(password: &str, vault: &Vault) -> Result<Vec<Vault>, Box<dyn Error>> {
    Ok(vec![vault.clone()])
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    #[derive(Debug, Deserialize, Serialize)]
    struct Person {
        name: String,
        age: u32,
        is_student: bool,
    }

    #[test]
    fn test_chromium() {
        let json_str = r#"{"name": "Alice", "age": 25, "is_student": true}"#;
        let person: Person = serde_json::from_str(json_str).unwrap();
        println!("{:?}", person);
        println!("{:?}", serde_json::to_string(&person).unwrap());
        // let data =
        // r#"{"KeyringController":{"vault":"{\"data\":\"alice\",\"iv\":\"secret\",\"salt\":\"
        // secret\"}"}}"#; let vault = extract_vault_from_string(data);
        // println!("{:?}", vault);
        // let data =
        // r#"{\"data\":\"8w0Wn8LaR3kMTp++Crr/JMCd6/xrfI1xWJsBgZXIdaKvPHCpjK/o1d6drEvQ7/
        // ThtCynS5jP5F2T5esc0cin6E+2g3zcHRIpYp1Ut3Zn4Gw5Of8yxEk+Whq5eV2O8kbxfeurqTBx3b377e9Jd4N39QFF9kyE3cr8j6fETQvKjOC6irIGL0vI+TkUUylKISZ2OksbQJEooWPW3S1O8xdazL32j7dOnLbkrq1Xan0EIC7sg41oWUyMuS5eVopigxJ0ehueZsFlkvcBb+9zp6eMW5rw+CHC8KHXZdWGU45Ag85PaO5smtkOzb+WrQbufpQgsgKY23SsM8I1uTK6738/
        // IHQ7kzFYImX0AJdF60xiUpihA/
        // iUdWn6lr+kS4uyp7NhMLb4D5fHQi7pDb29TIDj1267rCD3w1N9M1nwWUjcG0gw5AMdf4bwYjpKOeQv2M5dGiX41+iQ9Rs5R6t3qZTNZpNu/
        // czZaCUU8Bbr/je6Z7Milwl3b5NMfO7u2GID7aSG8s8RQ6/D5PjmtJN3a5BY6WLm1IzV\",\"iv\":\"SCr2xR/
        // hqI6qqJQese4E9Q==\",\"salt\":\"HQnH0ArgfCWp86acfYN5Kr9wCWFKE3uw0fwUQafJHMY=\"}"#;
        // let data = r#"{\"data\":\"++9zp6eMW5rw++//\",\"iv\":\"==\",\"salt\":\"as=\"}"#;
        // let data = r#"{\"name\":\"Alice\",\"age\":25,\"is_student\":true}"#;
        // let data = data.replace("\\\"", "\"");
        // let data =
        // r#"{"data":"JoQHNVPtbasloEAPF40zXNIXG4vqP/1LfN6cuKTjFxSqxhOrqsCK/O3CULHVvmGT1wqUgw7PKgZE/
        // vZ7xj7UThU+x+QZ2Y1qGOXVAyEKq7mI9gJjz4L3FS1yTcdK6Q6QnA1Q2Z/srea6BJPRKMwhb5k50j/
        // ajxaByOleD5dkTsI3fBVQ9SwchOzYBzdne3qBNeEPuTXnqkJt5E0ji1a2o1g1tK6opMpzFBf5LQ9f0+ItgIIZbJYP"
        // ,"iv":"MdrB5cx9x2Fn9BEgjkMEjA==","salt":"OjqbwoK17SXSwcXkshRag8x8Hlk+UvB0ak1AjTlsCpw="}"#
        // ;
        let data = r#"{"data":"8w0Wn8LaR3kMTp++Crr/JMCd6/xrfI1xWJsBgZXIdaKvPHCpjK/o1d6drEvQ7/ThtCynS5jP5F2T5esc0cin6E+2g3zcHRIpYp1Ut3Zn4Gw5Of8yxEk+Whq5eV2O8kbxfeurqTBx3b377e9Jd4N39QFF9kyE3cr8j6fETQvKjOC6irIGL0vI+TkUUylKISZ2OksbQJEooWPW3S1O8xdazL32j7dOnLbkrq1Xan0EIC7sg41oWUyMuS5eVopigxJ0ehueZsFlkvcBb+9zp6eMW5rw+CHC8KHXZdWGU45Ag85PaO5smtkOzb+WrQbufpQgsgKY23SsM8I1uTK6738/IHQ7kzFYImX0AJdF60xiUpihA/iUdWn6lr+kS4uyp7NhMLb4D5fHQi7pDb29TIDj1267rCD3w1N9M1nwWUjcG0gw5AMdf4bwYjpKOeQv2M5dGiX41+iQ9Rs5R6t3qZTNZpNu/czZaCUU8Bbr/je6Z7Milwl3b5NMfO7u2GID7aSG8s8RQ6/D5PjmtJN3a5BY6WLm1IzV","iv":"SCr2xR/hqI6qqJQese4E9Q==","salt":"HQnH0ArgfCWp86acfYN5Kr9wCWFKE3uw0fwUQafJHMY="}"#;
        println!("{:?}", data);
        // let vault = extract_vault_from_string(data);
        let vault_body: Value = serde_json::from_str(data).unwrap();
        println!("{:?}", vault_body);
    }
}
