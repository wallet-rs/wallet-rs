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
        string[1..index].to_string()
    } else {
        string.to_string()
    }
}

/// Get the regex string from the enum
pub fn get_regex(keyword: RegexEnum) -> String {
    let regex = MY_MAP.get(&keyword).cloned().unwrap();
    // Replace { with \{
    let regex = regex.replace(r#"{""#, r#"\{""#);
    // Replace :("{[ with :("\{[
    let regex = regex.replace(r#":("{["#, r#":("\{["#);
    // Replace :"{[^ with :"\{[^
    let regex = regex.replace(r#":"{[^"#, r#":"\{[^"#);
    // Replace \/] with \\/]
    #[allow(clippy::all)]
    let regex = regex.replace(r#"\/]"#, r#"\\/]"#);
    parse_regex_rust(&regex)
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
            re: r#"\{"wallet-seed":"([^"}]*)""#,
        },
        Regex {
            regex: RegexEnum::WalletV2,
            // /"wallet":("{[ -~]*\\"version\\":2}")/
            be: r#""wallet":("{[ -~]*\\"version\\":2}")"#,
            re: r#""wallet":("\{[ -~]*\\"version\\":2}")"#,
        },
        Regex {
            regex: RegexEnum::Keyring,
            // /"KeyringController":{"vault":"{[^{}]*}"/)
            be: r#""KeyringController":{"vault":"{[^{}]*}""#,
            re: r#""KeyringController":\{"vault":"\{[^{}]*}""#,
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
            re: r#"\\"iv.{1,4}[^A-Za-z0-9+\\/]{1,10}([A-Za-z0-9+\\/]{10,40}=*)"#,
        },
        Regex {
            regex: RegexEnum::DataRegex,
            // /\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)/u
            be: r#"\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)"#,
            re: r#"\\"[^":,is]*\\":\\"([A-Za-z0-9+\\/]*=*)"#,
        },
        Regex {
            regex: RegexEnum::SaltRegex,
            //  /,\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)/u
            be: r#",\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)"#,
            re: r#",\\"salt.{1,4}[^A-Za-z0-9+\\/]{1,10}([A-Za-z0-9+\\/]{10,100}=*)"#,
        },
    ];

    #[test]
    fn test_get_regex() {
        for fixture in FIXTURES.iter() {
            // First check the original regex string
            let be = parse_regex(fixture.regex.clone());
            assert_eq!(be, fixture.be);

            // Then check the parsed regex string
            let re = get_regex(fixture.regex.clone());
            let _ = regex::Regex::new(&re).unwrap();
            assert_eq!(re, fixture.re);
        }
    }

    #[test]
    fn test_match_regex() {
        let match_regex = regex::Regex::new(&get_regex(RegexEnum::MatchRegex)).unwrap();
        let target = r##""TypAwnona gL.D},"IncomingTransacAN:ninF" i xAFetc!�0lockByChainId-�<":14713724,"0x2a)� 3 4 5	�Keyring6� vac":"{\"���;\":\"ZQx4MOjDWxI4dq6eC9GO6jCkZPcOJ1QR7c6zgjif+9u4WcvCxgemuh94N/n8jVjJl/Q7yMI7uQtdHuKhq+9kA3sm+7yf1RAEEUq+MRnRBp694dYBurovm8qF2BrmRmtrVoT5DgmoaDzw1GOwrn1KNtdyp/tgi6wRUDLx7Ntotlb1AUm7lf3MQgmSABcUy6zcdAOBFxGXPHjJ38Y1qGfTQGA2XmkyqpLOKhga2HfNAQnksbCA6zrlw0W8zh6P2eu8GCLkIM2YqC7llK8mVm3Itfln5KsDgtNgg86avAF1a4chYfFsGN5RGwzV41OJo06j1v5G05AFMFSLLakfFQeT3iHNP/pxc0pEcb16HRiezP72N5vAU2rEVre17oJXenHXCt2/Zh6lmNYWCz6kJ3yH7T+SprafzVuJPc2GHBLPjUQ976Gf27bmfUl3vBnLd9AiL0GsRPoYao2mS1Qv8Y9SEGx113nYL2jCGE8++d0hCa1qcMIo2o/an2ZA38Fg5J46H1BLCSghFGTAcyFFQYw6MrSiieCcHgdNw4GlXTILooCxlsZK8KszaplKOq9eTwAUZ1YzVl1yDRdi8O9Dwz6tIFsu5nFc+1O0C5WT9AeSLh+aM/84ybIDsJCWk+kMZNRp5CEFr3Yxyd7x25O9/ztYSrulYVcX+9vc0APX89QBfXjlMFn9RNO0O/wtuQ/9RG+A57ExcCYOWcGAyt4+B9jhP9psRf+3rirNcuZHWMXSVzaKY5nEQcvKUNTgk06CcWu8RQTMPrUzl73eUELsdCS9ic3KpaRfKC7+O4FZAKKESWKBTTE=\",\"ive<�3hGyBrQyhi8uBU4yW5vlkw==\",\"salt&�b99kHOENj+LW2VItPxI9fvQt6ixt+qKtceoIns7Czlc=\"}�$MetaMetric:Vfrag�ȍK m(�5�R"0x56ae23014f4b37cb1fa517d4de294a37f7cd9c174474820cb652cc7b759ab585","participateIn��x$},"Network6\ n	":"1��	Detail�"EIPS���"##;
        let matches = match_regex.find_iter(target);
        assert!(matches.count() > 0);
    }
}
