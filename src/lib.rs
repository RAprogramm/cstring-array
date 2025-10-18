// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Safe, zero-copy wrapper for passing string arrays to C FFI (`char**`)
//!
//! This crate provides [`CStringArray`], a safe abstraction over C's
//! null-terminated string arrays, commonly used for command-line arguments and
//! similar purposes.
//!
//! # Features
//!
//! - **Memory-safe**: RAII-based lifetime management prevents dangling pointers
//! - **Zero-copy**: When constructed from `Vec<CString>`, no re-allocation
//!   occurs
//! - **C-compatible**: Produces valid `char**` pointers with null termination
//! - **Ergonomic**: Multiple constructors and trait implementations for easy
//!   usage
//! - **Well-tested**: Comprehensive test coverage for reliability
//!
//! # Example
//!
//! ```
//! use std::ffi::c_char;
//!
//! use cstring_array::CStringArray;
//!
//! let args = vec![
//!     "program".to_string(),
//!     "--verbose".to_string(),
//!     "file.txt".to_string(),
//! ];
//! let array = CStringArray::new(args).unwrap();
//!
//! // Safe to pass to C FFI functions expecting char**
//! let ptr: *const *const c_char = array.as_ptr();
//! assert_eq!(array.len(), 3);
//! ```
//!
//! # Creating Arrays
//!
//! Multiple ways to construct a `CStringArray`:
//!
//! ```
//! use std::{convert::TryFrom, ffi::CString};
//!
//! use cstring_array::CStringArray;
//!
//! // From Vec<String>
//! let arr1 = CStringArray::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();
//!
//! // From Vec<CString> (zero-copy)
//! let cstrings = vec![CString::new("foo").unwrap(), CString::new("bar").unwrap()];
//! let arr2 = CStringArray::from_cstrings(cstrings).unwrap();
//!
//! // Using TryFrom with Vec<&str>
//! let arr3 = CStringArray::try_from(vec!["foo", "bar"]).unwrap();
//!
//! // Using TryFrom with arrays
//! let arr4 = CStringArray::try_from(["foo", "bar"]).unwrap();
//! ```
//!
//! # Safety Considerations
//!
//! The pointer returned by [`CStringArray::as_ptr`] is only valid for the
//! lifetime of the `CStringArray`. Ensure the array outlives any C code using
//! the pointer:
//!
//! ```
//! use std::ffi::c_char;
//!
//! use cstring_array::CStringArray;
//!
//! fn call_c_function(argv: *const *const c_char, argc: i32) {
//!     // ... FFI call ...
//! }
//!
//! let array = CStringArray::new(vec!["arg1".to_string(), "arg2".to_string()]).unwrap();
//! let ptr = array.as_ptr();
//! call_c_function(ptr, array.len() as i32);
//! // array must not be dropped before call_c_function returns
//! ```

mod array;
mod error;
mod traits;

#[cfg(test)]
mod tests;

pub use array::CStringArray;
pub use error::CStringArrayError;
