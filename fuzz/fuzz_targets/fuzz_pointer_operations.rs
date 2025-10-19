// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![no_main]

use std::ffi::CStr;

use cstring_array::CStringArray;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }

    let num_strings = (data[0] as usize % 100) + 1;
    let index = data[1] as usize;

    let strings: Vec<String> = (0..num_strings).map(|i| format!("string_{}", i)).collect();

    if let Ok(array) = CStringArray::new(strings) {
        let _ = array.len();
        let _ = array.is_empty();
        let ptr = array.as_ptr();

        unsafe {
            for i in 0..array.len() {
                let cstr_ptr = *ptr.offset(i as isize);
                if !cstr_ptr.is_null() {
                    let _ = CStr::from_ptr(cstr_ptr);
                }
            }
        }

        let _ = array.get(index);

        for s in array.iter() {
            let _ = s.to_str();
        }
    }
});
