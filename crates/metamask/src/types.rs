// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Vault {
    pub data: String,
    pub iv: String,
    pub salt: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrBytes {
    String(String),
    Bytes(Vec<u8>),
}

impl std::fmt::Display for StringOrBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StringOrBytes::String(s) => write!(f, "{}", s),
            StringOrBytes::Bytes(_) => Ok(()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MnemoicData {
    pub mnemonic: StringOrBytes,
    pub number_of_accounts: Option<u32>,
    pub hd_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptedVault {
    pub r#type: Option<String>,
    pub data: MnemoicData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_or_bytes_display() {
        let string = StringOrBytes::String("hello".to_string());
        let bytes = StringOrBytes::Bytes(vec![0x68, 0x65, 0x6c, 0x6c, 0x6f]);

        assert_eq!(format!("{}", string), "hello");
        assert_eq!(format!("{}", bytes), "");
    }
}
