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
pub struct MnemoicData {
    pub mnemonic: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecryptedVault {
    pub r#type: Option<String>,
    pub data: MnemoicData,
    pub number_of_accounts: Option<u32>,
    pub hd_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnotherMnemoicData {
    pub mnemonic: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnotherDecryptedVault {
    pub r#type: Option<String>,
    pub data: AnotherMnemoicData,
    pub number_of_accounts: Option<u32>,
    pub hd_path: Option<String>,
}
