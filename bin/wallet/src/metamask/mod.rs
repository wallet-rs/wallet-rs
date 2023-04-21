// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use clap::Parser;
use wallet_metamask::interactive::locate_metamask_extension;

/// Start the node
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub async fn run(&self) -> eyre::Result<()> {
        let a = locate_metamask_extension();
        if let Err(a) = a {
            let err = eyre::eyre!("Error while finding MetaMask extension: {}", a);
            return Err(err);
        }
        println!("Found MetaMask extension: {:?}", a);
        Ok(())
    }
}
