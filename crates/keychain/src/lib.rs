// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub fn rust_greeting(to: String) -> String {
    format!("Hello World, {}!", to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_greeting() {
        assert_eq!(rust_greeting("Rust".to_string()), "Hello World, Rust!".to_string());
    }
}
