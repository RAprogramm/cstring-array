// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use cstring_array::CStringArray;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let strings: Vec<String> =
            input.split('\n').take(10000).map(String::from).collect();

        if !strings.is_empty() {
            let _ = CStringArray::new(strings);
        }
    }
});
