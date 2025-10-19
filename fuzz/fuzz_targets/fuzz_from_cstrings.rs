// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use std::ffi::CString;

use cstring_array::CStringArray;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let cstrings: Result<Vec<CString>, _> =
            input.split('\n').take(10000).map(CString::new).collect();

        if let Ok(cstrings) = cstrings {
            if !cstrings.is_empty() {
                let _ = CStringArray::from_cstrings(cstrings);
            }
        }
    }
});
