// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Integration tests for CStringArray FFI compatibility.
//!
//! This module tests real-world FFI scenarios including:
//! - Passing arrays to mock C functions
//! - Verifying pointer validity and null termination
//! - Testing concurrent array usage
//! - Validating lifetime safety
//! - Real command-line argument patterns

use std::{
    convert::TryFrom,
    ffi::{CStr, CString, c_char, c_int}
};

use cstring_array::CStringArray;

extern "C" fn mock_c_function_count_args(argc: c_int, argv: *const *const c_char) -> c_int {
    assert!(!argv.is_null());
    argc
}

extern "C" fn mock_c_function_read_args(argc: c_int, argv: *const *const c_char) -> *const c_char {
    assert!(!argv.is_null());
    assert!(argc > 0);

    unsafe {
        assert!(!(*argv).is_null());
        *argv
    }
}

extern "C" fn mock_c_function_verify_null_termination(argv: *const *const c_char) -> c_int {
    let mut count = 0;
    unsafe {
        let mut ptr = argv;
        while !(*ptr).is_null() {
            count += 1;
            ptr = ptr.offset(1);
        }
    }
    count
}

#[test]
fn integration_basic_ffi_call() {
    let args = vec![
        "program".to_string(),
        "--flag".to_string(),
        "value".to_string(),
    ];
    let array = CStringArray::new(args).unwrap();

    let result = mock_c_function_count_args(array.len() as c_int, array.as_ptr());
    assert_eq!(result, 3);
}

#[test]
fn integration_read_first_arg() {
    let args = vec!["myprogram".to_string(), "arg1".to_string()];
    let array = CStringArray::new(args).unwrap();

    let first_ptr = mock_c_function_read_args(array.len() as c_int, array.as_ptr());
    let first_str = unsafe { CStr::from_ptr(first_ptr).to_str().unwrap() };

    assert_eq!(first_str, "myprogram");
}

#[test]
fn integration_null_termination() {
    let args = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let array = CStringArray::new(args).unwrap();

    let count = mock_c_function_verify_null_termination(array.as_ptr());
    assert_eq!(count, 3);
}

#[test]
fn integration_iterate_all_args() {
    let expected = vec!["prog", "arg1", "arg2", "arg3"];
    let args: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
    let array = CStringArray::new(args).unwrap();

    let ptr = array.as_ptr();
    let mut collected = Vec::new();

    unsafe {
        let mut current = ptr;
        while !(*current).is_null() {
            let s = CStr::from_ptr(*current).to_str().unwrap();
            collected.push(s.to_string());
            current = current.offset(1);
        }
    }

    assert_eq!(collected, expected);
}

#[test]
fn integration_try_from_various_sources() {
    let from_vec_str = CStringArray::try_from(vec!["a", "b"]).unwrap();
    assert_eq!(from_vec_str.len(), 2);

    let from_array = CStringArray::try_from(["x", "y", "z"]).unwrap();
    assert_eq!(from_array.len(), 3);

    let cstrings = vec![CString::new("m").unwrap(), CString::new("n").unwrap()];
    let from_cstrings = CStringArray::try_from(cstrings).unwrap();
    assert_eq!(from_cstrings.len(), 2);
}

#[test]
fn integration_from_array_strings() {
    let strings = ["one", "two", "three"];
    let array = CStringArray::try_from(strings).unwrap();
    assert_eq!(array.len(), 3);

    let ptr = array.as_ptr();
    unsafe {
        assert_eq!(CStr::from_ptr(*ptr).to_str().unwrap(), "one");
        assert_eq!(CStr::from_ptr(*ptr.offset(1)).to_str().unwrap(), "two");
        assert_eq!(CStr::from_ptr(*ptr.offset(2)).to_str().unwrap(), "three");
        assert!((*ptr.offset(3)).is_null());
    }
}

#[test]
fn integration_lifetime_safety() {
    let array = {
        let temp = vec!["scoped".to_string(), "data".to_string()];
        CStringArray::new(temp).unwrap()
    };

    let ptr = array.as_ptr();
    unsafe {
        assert_eq!(CStr::from_ptr(*ptr).to_str().unwrap(), "scoped");
    }
}

#[test]
fn integration_multiple_concurrent_arrays() {
    let arr1 = CStringArray::new(vec!["a1".to_string(), "a2".to_string()]).unwrap();
    let arr2 = CStringArray::new(vec!["b1".to_string(), "b2".to_string()]).unwrap();
    let arr3 = CStringArray::new(vec!["c1".to_string(), "c2".to_string()]).unwrap();

    let count1 = mock_c_function_verify_null_termination(arr1.as_ptr());
    let count2 = mock_c_function_verify_null_termination(arr2.as_ptr());
    let count3 = mock_c_function_verify_null_termination(arr3.as_ptr());

    assert_eq!(count1, 2);
    assert_eq!(count2, 2);
    assert_eq!(count3, 2);
}

#[test]
fn integration_real_world_command_line() {
    let command = "gcc";
    let flags = ["-O3", "-Wall", "-Werror"];
    let input_files = ["main.c", "utils.c"];
    let output = ["-o", "program"];

    let mut args = vec![command.to_string()];
    args.extend(flags.iter().map(|s| s.to_string()));
    args.extend(input_files.iter().map(|s| s.to_string()));
    args.extend(output.iter().map(|s| s.to_string()));

    let array = CStringArray::new(args).unwrap();
    assert_eq!(array.len(), 8);

    let ptr = array.as_ptr();
    unsafe {
        assert_eq!(CStr::from_ptr(*ptr).to_str().unwrap(), "gcc");
        assert_eq!(CStr::from_ptr(*ptr.offset(1)).to_str().unwrap(), "-O3");
        assert_eq!(CStr::from_ptr(*ptr.offset(7)).to_str().unwrap(), "program");
        assert!((*ptr.offset(8)).is_null());
    }
}

#[test]
fn integration_empty_strings_in_array() {
    let args = vec!["start".to_string(), "".to_string(), "end".to_string()];
    let array = CStringArray::new(args).unwrap();

    let ptr = array.as_ptr();
    unsafe {
        let first = CStr::from_ptr(*ptr).to_str().unwrap();
        let second = CStr::from_ptr(*ptr.offset(1)).to_str().unwrap();
        let third = CStr::from_ptr(*ptr.offset(2)).to_str().unwrap();

        assert_eq!(first, "start");
        assert_eq!(second, "");
        assert_eq!(third, "end");
    }
}

#[test]
fn integration_unicode_in_ffi() {
    let args = vec![
        "프로그램".to_string(),
        "файл".to_string(),
        "文件".to_string(),
    ];
    let array = CStringArray::new(args).unwrap();

    let count = mock_c_function_verify_null_termination(array.as_ptr());
    assert_eq!(count, 3);

    let ptr = array.as_ptr();
    unsafe {
        assert_eq!(CStr::from_ptr(*ptr).to_str().unwrap(), "프로그램");
        assert_eq!(CStr::from_ptr(*ptr.offset(1)).to_str().unwrap(), "файл");
        assert_eq!(CStr::from_ptr(*ptr.offset(2)).to_str().unwrap(), "文件");
    }
}
