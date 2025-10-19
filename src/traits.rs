// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Trait implementations for CStringArray.
//!
//! This module provides ergonomic conversions from various string collection
//! types into `CStringArray` through the `TryFrom` trait. These implementations
//! allow for flexible and convenient array construction from different input
//! formats.

use std::ffi::CString;

use crate::{array::CStringArray, error::CStringArrayError};

impl TryFrom<Vec<String>> for CStringArray {
    type Error = CStringArrayError;

    /// Converts a `Vec<String>` into a `CStringArray`.
    ///
    /// # Errors
    ///
    /// Returns an error if any string contains an interior null byte or if the
    /// vector is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let strings = vec!["hello".to_string(), "world".to_string()];
    /// let array = CStringArray::try_from(strings).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    fn try_from(strings: Vec<String>) -> Result<Self, Self::Error> {
        CStringArray::new(strings)
    }
}

impl TryFrom<Vec<&str>> for CStringArray {
    type Error = CStringArrayError;

    /// Converts a `Vec<&str>` into a `CStringArray`.
    ///
    /// # Errors
    ///
    /// Returns an error if any string contains an interior null byte or if the
    /// vector is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let strings = vec!["hello", "world"];
    /// let array = CStringArray::try_from(strings).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    fn try_from(strings: Vec<&str>) -> Result<Self, Self::Error> {
        let owned: Vec<String> = strings.into_iter().map(String::from).collect();
        CStringArray::new(owned)
    }
}

impl<const N: usize> TryFrom<[String; N]> for CStringArray {
    type Error = CStringArrayError;

    /// Converts an array of `String`s into a `CStringArray`.
    ///
    /// # Errors
    ///
    /// Returns an error if any string contains an interior null byte or if the
    /// array is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let strings = ["hello".to_string(), "world".to_string()];
    /// let array = CStringArray::try_from(strings).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    fn try_from(strings: [String; N]) -> Result<Self, Self::Error> {
        CStringArray::new(strings.to_vec())
    }
}

impl<const N: usize> TryFrom<[&str; N]> for CStringArray {
    type Error = CStringArrayError;

    /// Converts an array of string slices into a `CStringArray`.
    ///
    /// # Errors
    ///
    /// Returns an error if any string contains an interior null byte or if the
    /// array is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use std::convert::TryFrom;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let strings = ["hello", "world"];
    /// let array = CStringArray::try_from(strings).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    fn try_from(strings: [&str; N]) -> Result<Self, Self::Error> {
        let owned: Vec<String> = strings.into_iter().map(String::from).collect();
        CStringArray::new(owned)
    }
}

impl TryFrom<Vec<CString>> for CStringArray {
    type Error = CStringArrayError;

    /// Converts a `Vec<CString>` into a `CStringArray` (zero-copy).
    ///
    /// # Errors
    ///
    /// Returns an error if the vector is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use std::{convert::TryFrom, ffi::CString};
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let cstrings = vec![
    ///     CString::new("hello").unwrap(),
    ///     CString::new("world").unwrap(),
    /// ];
    /// let array = CStringArray::try_from(cstrings).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    fn try_from(strings: Vec<CString>) -> Result<Self, Self::Error> {
        CStringArray::from_cstrings(strings)
    }
}

// ============================================================================
// Comparison Traits
// ============================================================================

impl PartialEq for CStringArray {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for CStringArray {}

impl std::hash::Hash for CStringArray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

// ============================================================================
// Clone Trait
// ============================================================================

impl Clone for CStringArray {
    fn clone(&self) -> Self {
        Self::from_cstrings(self.as_slice().to_vec()).expect("clone from non-empty array")
    }
}

// ============================================================================
// Iterator Traits
// ============================================================================

impl FromIterator<String> for CStringArray {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let strings: Vec<String> = iter.into_iter().collect();
        Self::new(strings).expect("FromIterator from non-empty iterator")
    }
}

impl FromIterator<CString> for CStringArray {
    fn from_iter<I: IntoIterator<Item = CString>>(iter: I) -> Self {
        let strings: Vec<CString> = iter.into_iter().collect();
        Self::from_cstrings(strings).expect("FromIterator from non-empty iterator")
    }
}

impl IntoIterator for CStringArray {
    type Item = CString;
    type IntoIter = std::vec::IntoIter<CString>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_strings().into_iter()
    }
}

impl<'a> IntoIterator for &'a CStringArray {
    type Item = &'a CString;
    type IntoIter = std::slice::Iter<'a, CString>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// ============================================================================
// Indexing Traits
// ============================================================================

impl std::ops::Index<usize> for CStringArray {
    type Output = CString;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

// ============================================================================
// Conversion Traits
// ============================================================================

impl AsRef<[CString]> for CStringArray {
    fn as_ref(&self) -> &[CString] {
        self.as_slice()
    }
}
