// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Comprehensive unit tests for CStringArray.
//!
//! This module contains exhaustive tests covering all functionality including:
//! - Construction from various input types
//! - Error handling and validation
//! - Memory safety and pointer operations
//! - Unicode string support
//! - Edge cases and boundary conditions

use std::{convert::TryFrom, ffi::CString};

use crate::{CStringArray, CStringArrayError::*};

#[test]
fn test_new_from_strings() {
    let strings = vec!["hello".to_string(), "world".to_string()];
    let array = CStringArray::new(strings);
    assert!(array.is_ok());

    let array = array.unwrap();
    assert_eq!(array.len(), 2);
    assert!(!array.is_empty());
}

#[test]
fn test_new_from_empty_vec() {
    let strings: Vec<String> = vec![];
    let result = CStringArray::new(strings);
    assert!(result.is_err());

    match result {
        Err(EmptyArray) => {}
        _ => panic!("Expected EmptyArray error")
    }
}

#[test]
fn test_new_with_interior_null() {
    let strings = vec!["hello".to_string(), "wo\0rld".to_string()];
    let result = CStringArray::new(strings);
    assert!(result.is_err());

    match result {
        Err(NulError(e)) => {
            assert_eq!(e.nul_position(), 2);
        }
        _ => panic!("Expected NulError")
    }
}

#[test]
fn test_from_cstrings_zero_copy() {
    let cstrings = vec![CString::new("foo").unwrap(), CString::new("bar").unwrap()];
    let array = CStringArray::from_cstrings(cstrings);
    assert!(array.is_ok());

    let array = array.unwrap();
    assert_eq!(array.len(), 2);
}

#[test]
fn test_from_empty_cstrings() {
    let cstrings: Vec<CString> = vec![];
    let result = CStringArray::from_cstrings(cstrings);
    assert!(result.is_err());
}

#[test]
fn test_as_ptr_null_terminated() {
    let array = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let ptr = array.as_ptr();

    unsafe {
        assert!(!(*ptr).is_null());
        assert!(!(*ptr.offset(1)).is_null());
        assert!((*ptr.offset(2)).is_null());
    }
}

#[test]
fn test_as_ptr_valid_strings() {
    let array = CStringArray::new(vec!["hello".to_string(), "world".to_string()]).unwrap();
    let ptr = array.as_ptr();

    unsafe {
        let first = std::ffi::CStr::from_ptr(*ptr);
        let second = std::ffi::CStr::from_ptr(*ptr.offset(1));

        assert_eq!(first.to_str().unwrap(), "hello");
        assert_eq!(second.to_str().unwrap(), "world");
    }
}

#[test]
fn test_as_mut_ptr() {
    let mut array = CStringArray::new(vec!["test".to_string()]).unwrap();
    let ptr = array.as_mut_ptr();
    assert!(!ptr.is_null());

    unsafe {
        assert!(!(*ptr).is_null());
    }
}

#[test]
fn test_len_and_is_empty() {
    let array_empty_result = CStringArray::new(vec![]);
    assert!(array_empty_result.is_err());

    let array_one = CStringArray::new(vec!["x".to_string()]).unwrap();
    assert_eq!(array_one.len(), 1);
    assert!(!array_one.is_empty());

    let array_three =
        CStringArray::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]).unwrap();
    assert_eq!(array_three.len(), 3);
    assert!(!array_three.is_empty());
}

#[test]
fn test_get() {
    let array = CStringArray::new(vec!["first".to_string(), "second".to_string()]).unwrap();

    assert!(array.get(0).is_some());
    assert_eq!(array.get(0).unwrap().to_str().unwrap(), "first");

    assert!(array.get(1).is_some());
    assert_eq!(array.get(1).unwrap().to_str().unwrap(), "second");

    assert!(array.get(2).is_none());
    assert!(array.get(100).is_none());
}

#[test]
fn test_iter() {
    let array =
        CStringArray::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]).unwrap();

    let collected: Vec<&CString> = array.iter().collect();
    assert_eq!(collected.len(), 3);

    let strings: Vec<&str> = array.iter().map(|cs| cs.to_str().unwrap()).collect();
    assert_eq!(strings, vec!["a", "b", "c"]);
}

#[test]
fn test_from_vec_strings() {
    let strings = vec!["foo".to_string(), "bar".to_string()];
    let result = CStringArray::new(strings);
    assert!(result.is_ok());

    let array = result.unwrap();
    assert_eq!(array.len(), 2);
}

#[test]
fn test_from_vec_cstrings() {
    let cstrings = vec![
        CString::new("hello").unwrap(),
        CString::new("world").unwrap(),
    ];
    let result = CStringArray::from_cstrings(cstrings);
    assert!(result.is_ok());

    let array = result.unwrap();
    assert_eq!(array.len(), 2);
}

#[test]
fn test_try_from_vec_string() {
    let strings = vec!["hello".to_string(), "world".to_string()];
    let result = CStringArray::try_from(strings);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_try_from_vec_str() {
    let strings = vec!["hello", "world"];
    let result = CStringArray::try_from(strings);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_try_from_array_string() {
    let strings = ["hello".to_string(), "world".to_string()];
    let result = CStringArray::try_from(strings);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_try_from_array_str() {
    let strings = ["hello", "world"];
    let result = CStringArray::try_from(strings);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_try_from_vec_cstring() {
    let cstrings = vec![
        CString::new("hello").unwrap(),
        CString::new("world").unwrap(),
    ];
    let result = CStringArray::try_from(cstrings);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_try_from_with_null_byte() {
    let strings = vec!["hello", "wo\0rld"];
    let result = CStringArray::try_from(strings);
    assert!(result.is_err());
}

#[test]
fn test_send_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<CStringArray>();
    assert_sync::<CStringArray>();
}

#[test]
fn test_drop_cleanup() {
    {
        let _array = CStringArray::new(vec!["test".to_string(), "data".to_string()]).unwrap();
    }
}

#[test]
fn test_multiple_arrays_no_interference() {
    let array1 = CStringArray::new(vec!["a1".to_string(), "a2".to_string()]).unwrap();
    let array2 = CStringArray::new(vec!["b1".to_string(), "b2".to_string()]).unwrap();

    assert_eq!(array1.len(), 2);
    assert_eq!(array2.len(), 2);

    let ptr1 = array1.as_ptr();
    let ptr2 = array2.as_ptr();

    assert_ne!(ptr1, ptr2);

    unsafe {
        assert_eq!(std::ffi::CStr::from_ptr(*ptr1).to_str().unwrap(), "a1");
        assert_eq!(std::ffi::CStr::from_ptr(*ptr2).to_str().unwrap(), "b1");
    }
}

#[test]
fn test_large_array() {
    let strings: Vec<String> = (0..1000).map(|i| format!("string_{}", i)).collect();
    let array = CStringArray::new(strings).unwrap();

    assert_eq!(array.len(), 1000);

    for i in 0..1000 {
        let cstr = array.get(i).unwrap();
        assert_eq!(cstr.to_str().unwrap(), format!("string_{}", i));
    }
}

#[test]
fn test_unicode_strings() {
    let strings = vec![
        "Hello мир".to_string(),
        "你好世界".to_string(),
        "مرحبا بالعالم".to_string(),
    ];

    let array = CStringArray::new(strings.clone()).unwrap();
    assert_eq!(array.len(), 3);

    for (i, expected) in strings.iter().enumerate() {
        assert_eq!(array.get(i).unwrap().to_str().unwrap(), expected);
    }
}

#[test]
fn test_single_element_array() {
    let array = CStringArray::new(vec!["only".to_string()]).unwrap();
    assert_eq!(array.len(), 1);
    assert!(!array.is_empty());

    let ptr = array.as_ptr();
    unsafe {
        assert!(!(*ptr).is_null());
        assert!((*ptr.offset(1)).is_null());
    }
}

#[test]
fn test_error_display() {
    let empty_err = EmptyArray;
    assert_eq!(
        format!("{}", empty_err),
        "Cannot create array from empty input"
    );

    let strings = vec!["te\0st".to_string()];
    if let Err(e) = CStringArray::new(strings) {
        let display = format!("{}", e);
        assert!(display.contains("interior null byte"));
        assert!(display.contains("position 2"));
    } else {
        panic!("Expected error");
    }
}

#[test]
fn test_pointer_stability() {
    let array = CStringArray::new(vec!["stable".to_string()]).unwrap();
    let ptr1 = array.as_ptr();
    let ptr2 = array.as_ptr();
    assert_eq!(ptr1, ptr2);
}

#[test]
fn test_empty_strings_allowed() {
    let strings = vec!["".to_string(), "non-empty".to_string(), "".to_string()];
    let array = CStringArray::new(strings).unwrap();
    assert_eq!(array.len(), 3);

    assert_eq!(array.get(0).unwrap().to_str().unwrap(), "");
    assert_eq!(array.get(1).unwrap().to_str().unwrap(), "non-empty");
    assert_eq!(array.get(2).unwrap().to_str().unwrap(), "");
}
