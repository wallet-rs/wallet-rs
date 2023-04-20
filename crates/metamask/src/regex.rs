// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lazy_static::lazy_static;
use std::collections::HashMap;

/// Types of regex strings
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RegexEnum {
    WalletSeed,
    WalletV2,
    Keyring,
    MatchRegex,
    CaptureRegex,
    IVRegex,
    DataRegex,
    SaltRegex,
}

lazy_static! {
    /// A map of regex strings
    static ref MY_MAP: HashMap<RegexEnum, &'static str> = {
        let mut map = HashMap::new();

        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L33
        // const matches = data.match(/{"wallet-seed":"([^"}]*)"/)
        map.insert(RegexEnum::WalletSeed, r#"/{"wallet-seed":"([^"}]*)"/"#);
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L36
        // const vaultMatches = data.match(/"wallet":("{[ -~]*\\"version\\":2}")/)
        map.insert(RegexEnum::WalletV2, r#"/"wallet":("{[ -~]*\\"version\\":2}")/"#);
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L53
        // const matches = data.match(/"KeyringController":{"vault":"{[^{}]*}"/)
        map.insert(RegexEnum::Keyring, r#"/"KeyringController":{"vault":"{[^{}]*}"/"#);
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L64
        // const matchRegex = /Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu
        map.insert(RegexEnum::MatchRegex, r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu"#);
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L65
        // const captureRegex  = /Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/u
        map.insert(RegexEnum::CaptureRegex, r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/u"#);
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L66
        // const ivRegex = /\\"iv.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,40}=*)/u
        map.insert(
            RegexEnum::IVRegex,
            r#"/\\"iv.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,40}=*)/u"#,
        );
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L67
        // const dataRegex = /\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)/u
        map.insert(RegexEnum::DataRegex, r#"/\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)/u"#);
        // From:
        // https://github.com/MetaMask/vault-decryptor/blob/master/app/lib.js#L68
        // const saltRegex = /,\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)/u
        map.insert(
            RegexEnum::SaltRegex,
            r#"/,\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)/u"#,
        );
        map
    };
}

// Replace until the last forward slash
// Parse js regex string to rust regex string
// Handle regex options like /regex/gu
fn parse_regex_rust(string: &str) -> String {
    if let Some(index) = string.rfind('/') {
        // Remove first until the last forward slash
        dbg!(&string);
        string[1..index].to_string()
    } else {
        string.to_string()
    }
}
/// Get the regex string from the enum
pub fn get_regex(keyword: RegexEnum) -> String {
    let regex = MY_MAP.get(&keyword).cloned().unwrap();
    parse_regex_rust(regex)
}

#[cfg(test)]
#[test]
// Test the get_regex function
fn test_get_regex() {
    let regex = get_regex(RegexEnum::WalletSeed);
    // /{"wallet-seed":"([^"}]*)"/
    assert_eq!(regex, r#"{"wallet-seed":"([^"}]*)""#);

    let regex = get_regex(RegexEnum::WalletV2);
    // /"wallet":("{[ -~]*\\"version\\":2}")/
    assert_eq!(regex, r#""wallet":("{[ -~]*\\"version\\":2}")"#);

    let regex = get_regex(RegexEnum::Keyring);
    // /"KeyringController":{"vault":"{[^{}]*}"/)
    assert_eq!(regex, r#""KeyringController":{"vault":"{[^{}]*}""#);

    let regex = get_regex(RegexEnum::MatchRegex);
    // /Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu
    assert_eq!(regex, r#"Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})"#);
}
