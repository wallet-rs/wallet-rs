// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Code for interacting with the MetaMask extension.
///
/// From:
/// https://support.metamask.io/hc/en-us/articles/360018766351-How-to-use-the-Vault-Decryptor-with-the-MetaMask-Vault-Data
use crate::types::Vault;
use crate::vault::extract_vault_from_file;
use inquire::{Password, PasswordDisplayMode};
use os_info;
use std::{error::Error, fs, path::PathBuf};
use tracing::{debug, trace};
use whoami;

// Interactively get the password from the user
pub fn get_password() -> Result<String, Box<dyn Error>> {
    let name = Password::new("Your metamask password:")
        .with_display_mode(PasswordDisplayMode::Masked)
        .prompt()?;

    Ok(name)
}

// Find the metamask extension files on the system
pub fn locate_metamask_extension() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // For Windows
    let user_name = "USER_NAME";
    let path = format!("C:\\Users\\{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Local Extension Settings\\nkbihfbeogaeaoehlefnkodbefgpgknn", user_name);
    let win_chrome_vault_path = PathBuf::from(path);

    // For Mac
    let path = "/Users/USERNAME/Library/Application Support/Google/Chrome/Default/Local Extension Settings/nkbihfbeogaeaoehlefnkodbefgpgknn";
    let mac_chrome_vault_path = PathBuf::from(path.replace("USERNAME", &whoami::username()));

    // For Ubuntu
    let path = "/Users/USERNAME/.config/google-chrome/Default/Local Extension Settings/nkbihfbeogaeaoehlefnkodbefgpgknn";
    let ubuntu_chrome_vault_path = PathBuf::from(path.replace("USERNAME", &whoami::username()));

    // Detect if windows or mac, and set the path accordingly
    let path = match os_info::get().os_type() {
        os_info::Type::Windows => win_chrome_vault_path,
        os_info::Type::Macos => mac_chrome_vault_path,
        os_info::Type::Ubuntu => ubuntu_chrome_vault_path,
        _ => panic!("Unsupported OS: {:?}", os_info::get().os_type()),
    };

    // Check if the path is a directory
    if !path.exists() && !path.is_dir() {
        debug!("Could not find MetaMask extension at: {:?}", path);
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

// Extract all vaults from the extension files
pub fn extract_all_vaults() -> Result<Vec<Vault>, Box<dyn Error>> {
    let a = locate_metamask_extension()?;

    // Collect all vaults that are found
    let vaults: Vec<Vault> = a
        .iter()
        .filter_map(|a| {
            trace!("Attempting to decrypt vault: {:?}", a);

            // Attempt to extract the vault from the extension
            let vault = extract_vault_from_file(a);

            // Return the vault if it exists, continue otherwise
            if let Ok(vault) = vault {
                Some(vault)
            } else {
                None
            }
        })
        .collect();

    Ok(vaults)
}
