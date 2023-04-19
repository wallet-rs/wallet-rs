use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RegexEnum {
    WalletSeed,
    WalletV2,
    Keyring,
}

lazy_static! {
    static ref MY_MAP: HashMap<RegexEnum, &'static str> = {
        let mut map = HashMap::new();
        map.insert(RegexEnum::WalletSeed, r#"/{"wallet-seed":"([^"}]*)"/"#);
        map.insert(RegexEnum::WalletV2, r#"/"wallet":("{[ -~]*\\"version\\":2}")/"#);
        map.insert(RegexEnum::Keyring, r#"/"KeyringController":{"vault":"{[^{}]*}"/"#);
        map
    };
}

pub fn get_regex(keyword: RegexEnum) -> String {
    let regex = MY_MAP.get(&keyword).cloned().unwrap();
    let regex = regex.replace('{', "\\{");
    regex[1..regex.len() - 1].to_string()
}
