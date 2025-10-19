// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Property-based tests for CStringArray.
//!
//! These tests validate invariants that should hold for ALL inputs by
//! generating thousands of random test cases automatically.

use std::ffi::CString;

use cstring_array::CStringArray;
use proptest::prelude::*;

// ============================================================================
// Strategy Definitions
// ============================================================================

// Strategy for generating valid strings (no null bytes)
prop_compose! {
    fn valid_string()(s in "[^\0]{0,100}") -> String {
        s
    }
}

// Strategy for generating vectors of valid strings (non-empty)
prop_compose! {
    fn valid_strings()(strings in prop::collection::vec(valid_string(), 1..50)) -> Vec<String> {
        strings
    }
}

// Strategy for generating strings with null bytes
prop_compose! {
    fn string_with_null()(
        prefix in "[^\0]{0,50}",
        suffix in "[^\0]{0,50}"
    ) -> String {
        format!("{}\0{}", prefix, suffix)
    }
}

// ============================================================================
// Property Tests
// ============================================================================

// Property 1: Length Invariant
// For all valid strings, the array length equals the input length
#[test]
fn length_invariant() {
    proptest!(|(strings in valid_strings())| {
        let arr = CStringArray::new(strings.clone()).unwrap();
        prop_assert_eq!(arr.len(), strings.len());
    });
}

// Property 2: Null Termination
// For all arrays, the pointer array is null-terminated
#[test]
fn null_termination() {
    proptest!(|(strings in valid_strings())| {
        let arr = CStringArray::new(strings).unwrap();
        let ptr = arr.as_ptr();

        unsafe {
            let null_ptr = *ptr.offset(arr.len() as isize);
            prop_assert!(null_ptr.is_null());
        }
    });
}

// Property 3: Index Access In Bounds
// For all arrays and valid indices, get() returns Some
#[test]
fn get_in_bounds() {
    proptest!(|(strings in valid_strings())| {
        let arr = CStringArray::new(strings.clone()).unwrap();

        for i in 0..arr.len() {
            prop_assert!(arr.get(i).is_some());
        }
    });
}

// Property 4: Index Access Out of Bounds
// For all arrays and out-of-bounds indices, get() returns None
#[test]
fn get_out_of_bounds() {
    proptest!(|(strings in valid_strings(), offset in 1..100usize)| {
        let arr = CStringArray::new(strings).unwrap();
        let invalid_idx = arr.len() + offset;

        prop_assert!(arr.get(invalid_idx).is_none());
    });
}

// Property 5: Iterator Length
// For all arrays, iterator count equals array length
#[test]
fn iterator_length() {
    proptest!(|(strings in valid_strings())| {
        let arr = CStringArray::new(strings).unwrap();
        prop_assert_eq!(arr.iter().count(), arr.len());
    });
}

// Property 6: Empty Array Rejection
// Empty vectors always return an error
#[test]
fn empty_array_rejected() {
    let empty: Vec<String> = vec![];
    assert!(CStringArray::new(empty).is_err());
}

// Property 7: Null Byte Detection
// Strings containing null bytes are rejected
#[test]
fn null_byte_detected() {
    proptest!(|(invalid in string_with_null())| {
        let result = CStringArray::new(vec![invalid]);
        prop_assert!(result.is_err());
    });
}

// Property 8: TryFrom Equivalence
// TryFrom and new() produce equivalent results
#[test]
fn try_from_equivalence() {
    proptest!(|(strings in valid_strings())| {
        let from_new = CStringArray::new(strings.clone()).unwrap();
        let from_try = CStringArray::try_from(strings).unwrap();

        prop_assert_eq!(from_new.len(), from_try.len());

        for i in 0..from_new.len() {
            let s1 = from_new.get(i).unwrap();
            let s2 = from_try.get(i).unwrap();
            prop_assert_eq!(s1, s2);
        }
    });
}

// Property 9: Clone Independence
// Cloned arrays are independent (different pointers)
#[test]
fn clone_independence() {
    proptest!(|(strings in valid_strings())| {
        let arr1 = CStringArray::new(strings).unwrap();
        let arr2 = arr1.clone();

        prop_assert_ne!(arr1.as_ptr(), arr2.as_ptr());
        prop_assert_eq!(arr1, arr2);
    });
}

// Property 10: FromIterator Consistency
// FromIterator produces same result as new()
#[test]
fn from_iterator_consistency() {
    proptest!(|(strings in valid_strings())| {
        let arr1 = CStringArray::new(strings.clone()).unwrap();
        let arr2: CStringArray = strings.into_iter().collect();

        prop_assert_eq!(arr1, arr2);
    });
}

// Property 11: Round-trip Through Into Strings
// into_strings() preserves all CStrings
#[test]
fn round_trip_into_strings() {
    proptest!(|(strings in valid_strings())| {
        let cstrings: Vec<CString> = strings
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap())
            .collect();

        let arr = CStringArray::from_cstrings(cstrings.clone()).unwrap();
        let recovered = arr.into_strings();

        prop_assert_eq!(cstrings, recovered);
    });
}

// Property 12: Slice Consistency
// as_slice() returns consistent view
#[test]
fn slice_consistency() {
    proptest!(|(strings in valid_strings())| {
        let arr = CStringArray::new(strings.clone()).unwrap();
        let slice = arr.as_slice();

        prop_assert_eq!(slice.len(), arr.len());

        for (i, s) in slice.iter().enumerate() {
            prop_assert_eq!(Some(s), arr.get(i));
        }
    });
}
