// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lazy_static::lazy_static;
use std::collections::HashMap;

/// Types of regex strings
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    let st = parse_regex_rust(regex);
    dbg!(&st);
    st.replace('{', "\\{")
}

#[cfg(test)]
mod test {
    use super::*;

    /// Get the regex string from the enum
    fn parse_regex(keyword: RegexEnum) -> String {
        let regex = MY_MAP.get(&keyword).cloned().unwrap();
        parse_regex_rust(regex)
    }

    struct Regex<'a> {
        regex: RegexEnum,
        be: &'a str,
        re: &'a str,
    }

    const FIXTURES: [Regex; 8] = [
        Regex {
            regex: RegexEnum::WalletSeed,
            // /{"wallet-seed":"([^"}]*)"/
            be: r#"{"wallet-seed":"([^"}]*)""#,
            re: r#"{"wallet-seed":"([^"}]*)""#,
        },
        Regex {
            regex: RegexEnum::WalletV2,
            // /"wallet":("{[ -~]*\\"version\\":2}")/
            be: r#""wallet":("{[ -~]*\\"version\\":2}")"#,
            re: r#""wallet":("{[ -~]*\\"version\\":2}")"#,
        },
        Regex {
            regex: RegexEnum::Keyring,
            // /"KeyringController":{"vault":"{[^{}]*}"/)
            be: r#""KeyringController":{"vault":"{[^{}]*}""#,
            re: r#""KeyringController":{"vault":"{[^{}]*}""#,
        },
        Regex {
            regex: RegexEnum::MatchRegex,
            // /Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu
            be: r#"Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})"#,
            re: r#"Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})"#,
        },
        Regex {
            regex: RegexEnum::CaptureRegex,
            // /Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/u
            be: r#"Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})"#,
            re: r#"Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})"#,
        },
        Regex {
            regex: RegexEnum::IVRegex,
            // /\\"iv.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,40}=*)/u
            be: r#"\\"iv.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,40}=*)"#,
            re: r#"\\"iv.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,40}=*)"#,
        },
        Regex {
            regex: RegexEnum::DataRegex,
            // /\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)/u
            be: r#"\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)"#,
            re: r#"\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)"#,
        },
        Regex {
            regex: RegexEnum::SaltRegex,
            //  /,\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)/u
            be: r#",\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)"#,
            re: r#",\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)"#,
        },
    ];

    #[test]
    fn test_get_regex() {
        for fixture in FIXTURES.iter() {
            let regex = parse_regex(fixture.regex.clone());
            assert_eq!(regex, fixture.be);
        }
    }
}
