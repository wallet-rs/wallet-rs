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
        map.insert(RegexEnum::WalletSeed, r#"/{"wallet-seed":"([^"}]*)"/"#);
        map.insert(RegexEnum::WalletV2, r#"/"wallet":("{[ -~]*\\"version\\":2}")/"#);
        map.insert(RegexEnum::Keyring, r#"/"KeyringController":{"vault":"{[^{}]*}"/"#);
        map.insert(RegexEnum::MatchRegex, r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu"#);
        map.insert(RegexEnum::CaptureRegex, r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/u"#);
        map.insert(
            RegexEnum::IVRegex,
            r#"/\\"iv.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,40}=*)/u"#,
        );
        map.insert(RegexEnum::DataRegex, r#"/\\"[^":,is]*\\":\\"([A-Za-z0-9+\/]*=*)/u"#);
        map.insert(
            RegexEnum::SaltRegex,
            r#"/,\\"salt.{1,4}[^A-Za-z0-9+\/]{1,10}([A-Za-z0-9+\/]{10,100}=*)/u"#,
        );
        map
    };
}

/// Get the regex string from the enum
pub fn get_regex(keyword: RegexEnum) -> String {
    let regex = MY_MAP.get(&keyword).cloned().unwrap();
    let regex = regex.replace('{', "\\{");
    regex[1..regex.len() - 1].to_string()
}

#[cfg(test)]
#[test]
// Test the get_regex function
fn test_get_regex() {
    let regex = get_regex(RegexEnum::WalletSeed);
    assert_eq!(regex, r#"\{"wallet-seed":"([^"}]*)""#);
    let regex = get_regex(RegexEnum::Keyring);
    let reg = r#""KeyringController":\{"vault":"\{[^\{}]*}""#;
    assert_eq!(regex, reg);
    // let _ = regex::Regex::new(r#"/Keyring[0-9][^\}]*(\{[^\{\}]*\\"\})/gu"#).unwrap();
}
