// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;

fn main() {
    // Get the current directory
    let current_dir = env::current_dir().unwrap();

    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Find the index of the --output argument
    let output_index = args.iter().position(|arg| arg == "--output");

    // Get the value of the --output argument if it is present
    let output = output_index.and_then(|i| args.get(i + 1));

    // Generate the C header file using the specified output directory, or the current directory if
    // no output directory is specified
    if let Some(output) = output {
        let _ = cbindgen::Builder::new()
            .with_crate(current_dir)
            .generate()
            .unwrap()
            .write_to_file(output);
    } else {
        let _ = cbindgen::generate(current_dir);
    }
}
