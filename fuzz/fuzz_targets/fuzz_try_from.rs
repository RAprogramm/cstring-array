// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use std::{convert::TryFrom, ffi::CString};

use cstring_array::CStringArray;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let strings: Vec<&str> = input.split('\n').take(1000).collect();

        if !strings.is_empty() {
            let _ = CStringArray::try_from(strings.clone());

            let owned: Vec<String> = strings.iter().map(|s| s.to_string()).collect();
            let _ = CStringArray::try_from(owned);

            let cstrings: Result<Vec<CString>, _> =
                strings.iter().map(|s| CString::new(*s)).collect();

            if let Ok(cstrings) = cstrings {
                let _ = CStringArray::try_from(cstrings);
            }
        }
    }
});
