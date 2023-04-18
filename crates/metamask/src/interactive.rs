// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Code for interacting with the MetaMask extension.
///
/// From:
/// https://support.metamask.io/hc/en-us/articles/360018766351-How-to-use-the-Vault-Decryptor-with-the-MetaMask-Vault-Data
use os_info;
use std::{error::Error, fs, path::PathBuf};
use whoami;

pub fn locate_metamask_extension() -> Result<Vec<PathBuf>, Box<dyn Error>> {
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

    // Attempt to open all files w/ the extension .log
    let files = fs::read_dir(path).unwrap().filter_map(|entry| {
        let path = entry.unwrap().path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "log" || ext == "ldb" {
                    return Some(path);
                }
            }
        }
        None
    });

    // Return a vec of all files of full paths
    Ok(files.collect())
}