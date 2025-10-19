// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Tests for standard Rust trait implementations.

use std::{collections::HashMap, ffi::CString};

use cstring_array::CStringArray;

#[test]
fn test_clone() {
    let arr1 = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let arr2 = arr1.clone();

    assert_eq!(arr1, arr2);
    assert_eq!(arr1.len(), arr2.len());
    assert_ne!(arr1.as_ptr(), arr2.as_ptr());

    assert_eq!(arr1.get(0).unwrap().to_str().unwrap(), "a");
    assert_eq!(arr2.get(0).unwrap().to_str().unwrap(), "a");
}

#[test]
fn test_equality() {
    let arr1 = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let arr2 = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let arr3 = CStringArray::new(vec!["b".to_string(), "a".to_string()]).unwrap();
    let arr4 = CStringArray::new(vec!["a".to_string()]).unwrap();

    assert_eq!(arr1, arr2);
    assert_ne!(arr1, arr3);
    assert_ne!(arr1, arr4);
}

#[test]
fn test_hash() {
    let mut map = HashMap::new();
    let arr1 = CStringArray::new(vec!["key".to_string()]).unwrap();
    let arr2 = CStringArray::new(vec!["key".to_string()]).unwrap();

    map.insert(arr1.clone(), "value1");
    map.insert(arr2, "value2");

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&arr1), Some(&"value2"));
}

#[test]
fn test_from_iterator_string() {
    let arr: CStringArray = vec!["a", "b", "c"].into_iter().map(String::from).collect();

    assert_eq!(arr.len(), 3);
    assert_eq!(arr.get(0).unwrap().to_str().unwrap(), "a");
    assert_eq!(arr.get(1).unwrap().to_str().unwrap(), "b");
    assert_eq!(arr.get(2).unwrap().to_str().unwrap(), "c");
}

#[test]
fn test_from_iterator_cstring() {
    let arr: CStringArray = vec!["a", "b", "c"]
        .into_iter()
        .map(|s| CString::new(s).unwrap())
        .collect();

    assert_eq!(arr.len(), 3);
    assert_eq!(arr.get(0).unwrap().to_str().unwrap(), "a");
    assert_eq!(arr.get(1).unwrap().to_str().unwrap(), "b");
    assert_eq!(arr.get(2).unwrap().to_str().unwrap(), "c");
}

#[test]
fn test_into_iterator_owned() {
    let arr = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let collected: Vec<CString> = arr.into_iter().collect();

    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].to_str().unwrap(), "a");
    assert_eq!(collected[1].to_str().unwrap(), "b");
}

#[test]
fn test_into_iterator_ref() {
    let arr = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let collected: Vec<&CString> = (&arr).into_iter().collect();

    assert_eq!(collected.len(), 2);
    assert_eq!(collected[0].to_str().unwrap(), "a");
    assert_eq!(collected[1].to_str().unwrap(), "b");
}

#[test]
fn test_for_loop_ref() {
    let arr = CStringArray::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]).unwrap();
    let mut count = 0;

    for s in &arr {
        assert!(s.to_str().is_ok());
        count += 1;
    }

    assert_eq!(count, 3);
}

#[test]
fn test_for_loop_owned() {
    let arr = CStringArray::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]).unwrap();
    let mut count = 0;

    for s in arr {
        assert!(s.to_str().is_ok());
        count += 1;
    }

    assert_eq!(count, 3);
}

#[test]
fn test_index() {
    let arr = CStringArray::new(vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ])
    .unwrap();

    assert_eq!(arr[0].to_str().unwrap(), "first");
    assert_eq!(arr[1].to_str().unwrap(), "second");
    assert_eq!(arr[2].to_str().unwrap(), "third");
}

#[test]
#[should_panic]
fn test_index_out_of_bounds() {
    let arr = CStringArray::new(vec!["a".to_string()]).unwrap();
    let _ = &arr[10];
}

#[test]
fn test_as_ref() {
    let arr = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    let slice: &[CString] = arr.as_ref();

    assert_eq!(slice.len(), 2);
    assert_eq!(slice[0].to_str().unwrap(), "a");
    assert_eq!(slice[1].to_str().unwrap(), "b");
}

#[test]
fn test_as_slice() {
    let arr = CStringArray::new(vec!["x".to_string(), "y".to_string(), "z".to_string()]).unwrap();
    let slice = arr.as_slice();

    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0].to_str().unwrap(), "x");
    assert_eq!(slice[1].to_str().unwrap(), "y");
    assert_eq!(slice[2].to_str().unwrap(), "z");
}

#[test]
fn test_into_strings() {
    let arr = CStringArray::new(vec!["test".to_string(), "data".to_string()]).unwrap();
    let strings = arr.into_strings();

    assert_eq!(strings.len(), 2);
    assert_eq!(strings[0].to_str().unwrap(), "test");
    assert_eq!(strings[1].to_str().unwrap(), "data");
}

#[test]
fn test_clone_and_modify() {
    let arr1 = CStringArray::new(vec!["original".to_string()]).unwrap();
    let arr2 = arr1.clone();

    assert_eq!(arr1, arr2);

    let arr3 = CStringArray::new(vec!["modified".to_string()]).unwrap();
    assert_ne!(arr1, arr3);
    assert_ne!(arr2, arr3);
}

#[test]
fn test_collect_from_filter() {
    let arr: CStringArray = vec!["a", "bb", "ccc", "dddd"]
        .into_iter()
        .filter(|s| s.len() > 1)
        .map(String::from)
        .collect();

    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].to_str().unwrap(), "bb");
    assert_eq!(arr[1].to_str().unwrap(), "ccc");
    assert_eq!(arr[2].to_str().unwrap(), "dddd");
}

#[test]
fn test_collect_from_map() {
    let arr: CStringArray = (1..=3).map(|i| format!("item{}", i)).collect();

    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].to_str().unwrap(), "item1");
    assert_eq!(arr[1].to_str().unwrap(), "item2");
    assert_eq!(arr[2].to_str().unwrap(), "item3");
}
