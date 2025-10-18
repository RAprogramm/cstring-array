// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Error types for CStringArray operations.
//!
//! This module provides the error types used throughout the crate when string array
//! operations fail. All errors implement the standard `Error` trait for proper error
//! handling and propagation.

use std::{
    error::Error,
    ffi::NulError,
    fmt::{Display, Formatter, Result as FmtResult}
};

#[cfg(test)]
use std::ffi::CString;

/// Error type for CStringArray operations
#[derive(Debug)]
pub enum CStringArrayError {
    /// String contains an interior null byte
    NulError(NulError),
    /// Empty string array is not allowed
    EmptyArray
}

impl Display for CStringArrayError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use CStringArrayError::*;
        match self {
            NulError(e) => {
                write!(f, "String contains interior null byte at position {}", e.nul_position())
            }
            EmptyArray => write!(f, "Cannot create array from empty input")
        }
    }
}

impl Error for CStringArrayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use CStringArrayError::*;
        match self {
            NulError(e) => Some(e),
            EmptyArray => None
        }
    }
}

impl From<NulError> for CStringArrayError {
    fn from(err: NulError) -> Self {
        Self::NulError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nul_error_display() {
        let test_string = "hello\0world";
        let nul_err = match CString::new(test_string) {
            Err(e) => e,
            Ok(_) => panic!("Expected NulError")
        };

        let err = CStringArrayError::from(nul_err);
        let display = format!("{}", err);
        assert!(display.contains("interior null byte"));
        assert!(display.contains("position 5"));
    }

    #[test]
    fn test_empty_array_display() {
        use CStringArrayError::*;
        let err = EmptyArray;
        assert_eq!(format!("{}", err), "Cannot create array from empty input");
    }

    #[test]
    fn test_error_source_nul() {
        let test_string = "test\0data";
        let nul_err = CString::new(test_string).unwrap_err();
        let err = CStringArrayError::from(nul_err);

        assert!(err.source().is_some());
        let source = err.source().unwrap();
        assert!(source.is::<NulError>());
    }

    #[test]
    fn test_error_source_empty() {
        use CStringArrayError::*;
        let err = EmptyArray;
        assert!(err.source().is_none());
    }

    #[test]
    fn test_from_nul_error() {
        use CStringArrayError::*;
        let nul_err = CString::new("a\0b").unwrap_err();
        let err: CStringArrayError = nul_err.into();

        match err {
            NulError(e) => {
                assert_eq!(e.nul_position(), 1);
            }
            _ => panic!("Expected NulError variant")
        }
    }

    #[test]
    fn test_debug_format() {
        use CStringArrayError::*;
        let err1 = EmptyArray;
        let debug_str = format!("{:?}", err1);
        assert!(debug_str.contains("EmptyArray"));

        let nul_err = CString::new("x\0y").unwrap_err();
        let err2 = CStringArrayError::from(nul_err);
        let debug_str2 = format!("{:?}", err2);
        assert!(debug_str2.contains("NulError"));
    }
}
