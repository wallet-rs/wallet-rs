// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();

    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Find the index of the --output argument
    let output_dir_index = args.iter().position(|arg| arg == "--output");

    // Get the value of the --output-dir argument if it is present
    let output_dir = output_dir_index.and_then(|i| args.get(i + 1));

    // Generate the C header file using the specified output directory, or the current directory if
    // no output directory is specified
    if let Some(output_dir) = output_dir {
        let _ = cbindgen::Builder::new()
            .with_crate(current_dir)
            .generate()
            .unwrap()
            .write_to_file(output_dir);
    } else {
        let _ = cbindgen::generate(current_dir);
    }
}
