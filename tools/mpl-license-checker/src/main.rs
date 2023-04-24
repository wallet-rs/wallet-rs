// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use glob::glob;
use std::fs;

fn main() {
    let dirs = [
        "bin/**/src/**/*.rs",
        "bin/**/tests/**/*.rs",
        "crates/**/src/**/*.rs",
        "crates/**/tests/**/*.rs",
        "tools/**/src/**/*.rs",
        "tools/**/tests/**/*.rs",
    ];
    let mut failed = false;
    for dir in dirs.iter() {
        glob(dir).expect("Failed to read glob pattern").for_each(|entry| {
            if let Ok(path) = entry {
                let contents = fs::read_to_string(&path).expect("Failed to read file");
                if !contents.starts_with(
                    "// This Source Code Form is subject to the terms of the Mozilla Public\n\
                     // License, v. 2.0. If a copy of the MPL was not distributed with this\n\
                     // file, You can obtain one at https://mozilla.org/MPL/2.0/.\n",
                ) {
                    println!("License header not found in file: {:?}", path);
                    failed = true;
                }
            }
        });
    }
    if failed {
        panic!("License header check failed");
    }
}
