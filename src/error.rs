// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

#[cfg(test)]
use std::ffi::CString;
use std::ffi::NulError;

/// Error type for CStringArray operations
#[derive(Debug)]
pub enum CStringArrayError {
    /// String contains an interior null byte
    NulError(NulError),
    /// Empty string array is not allowed
    EmptyArray
}

impl std::fmt::Display for CStringArrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CStringArrayError::NulError(e) => {
                write!(
                    f,
                    "String contains interior null byte at position {}",
                    e.nul_position()
                )
            }
            CStringArrayError::EmptyArray => write!(f, "Cannot create array from empty input")
        }
    }
}

impl std::error::Error for CStringArrayError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CStringArrayError::NulError(e) => Some(e),
            CStringArrayError::EmptyArray => None
        }
    }
}

impl From<NulError> for CStringArrayError {
    fn from(err: NulError) -> Self {
        CStringArrayError::NulError(err)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

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
        let err = CStringArrayError::EmptyArray;
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
        let err = CStringArrayError::EmptyArray;
        assert!(err.source().is_none());
    }

    #[test]
    fn test_from_nul_error() {
        let nul_err = CString::new("a\0b").unwrap_err();
        let err: CStringArrayError = nul_err.into();

        match err {
            CStringArrayError::NulError(e) => {
                assert_eq!(e.nul_position(), 1);
            }
            _ => panic!("Expected NulError variant")
        }
    }

    #[test]
    fn test_debug_format() {
        let err1 = CStringArrayError::EmptyArray;
        let debug_str = format!("{:?}", err1);
        assert!(debug_str.contains("EmptyArray"));

        let nul_err = CString::new("x\0y").unwrap_err();
        let err2 = CStringArrayError::from(nul_err);
        let debug_str2 = format!("{:?}", err2);
        assert!(debug_str2.contains("NulError"));
    }
}
