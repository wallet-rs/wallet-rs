// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Code for interacting with the MetaMask extension.
///
/// From:
/// https://support.metamask.io/hc/en-us/articles/360018766351-How-to-use-the-Vault-Decryptor-with-the-MetaMask-Vault-Data
use os_info;
use std::{error::Error, path::PathBuf};
use whoami;

pub fn locate_metamask_extension() -> Result<PathBuf, Box<dyn Error>> {
    // For Windows
    let user_name = "USER_NAME";
    let path = format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Local Extension Settings\\nkbihfbeogaeaoehlefnkodbefgpgknn", user_name);
    let win_chrome_vault_path = PathBuf::from(path);

    // For Mac
    let path = "/Users/USERNAME/Library/Application Support/Google/Chrome/Default/Local Extension Settings/nkbihfbeogaeaoehlefnkodbefgpgknn";
    let mac_chrome_vault_path = PathBuf::from(path.replace("USERNAME", &whoami::username()));

    // Detect if windows or mac
    let path = if os_info::get().os_type() == os_info::Type::Windows {
        win_chrome_vault_path
    } else {
        mac_chrome_vault_path
    };

    // Check if the path is a directory
    if !path.exists() && !path.is_dir() {
        println!("Could not find MetaMask extension at: {:?}", path);
        return Err("Could not find MetaMask extension".into());
    }

    let possible_file_names =
        ["000003.ldb", "000004.ldb", "000005.ldb", "000003.log", "000004.log", "000005.log"];
    // Attempt to open the vault, cycle one of multiple possible filenames
    for file_name in possible_file_names.iter() {
        let vault_path = path.join(file_name);
        if vault_path.exists() {
            println!("Found MetaMask vault at: {:?}", vault_path);
            return Ok(vault_path);
        }
    }

    Err("Could not find MetaMask vault".into())
}
