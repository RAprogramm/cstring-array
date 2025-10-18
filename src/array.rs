// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::{
    ffi::{CString, c_char},
    ptr::null,
    slice::Iter
};

use crate::error::{CStringArrayError, CStringArrayError::EmptyArray};

/// Safe wrapper for passing string arrays to C FFI as `char**`.
///
/// This structure provides a safe abstraction over the common C pattern of
/// null-terminated arrays of strings (`char**`), often used for command-line
/// arguments (`argv`).
///
/// # Memory Safety
///
/// - Guarantees proper memory layout compatible with C's `char**`
/// - Automatically manages lifetime of all C strings
/// - Ensures null-termination of the pointer array
/// - Prevents dangling pointers through RAII
/// - Zero-copy when constructed from `Vec<CString>`
///
/// # Example
///
/// ```
/// use std::ffi::c_char;
///
/// use cstring_array::CStringArray;
///
/// let args = vec![
///     "program".to_string(),
///     "--verbose".to_string(),
///     "file.txt".to_string(),
/// ];
/// let array = CStringArray::new(args).unwrap();
///
/// // Safe to pass to C FFI
/// let ptr: *const *const c_char = array.as_ptr();
/// assert_eq!(array.len(), 3);
/// ```
#[derive(Debug)]
pub struct CStringArray {
    strings:  Vec<CString>,
    pointers: Vec<*const c_char>
}

impl CStringArray {
    /// Creates a new `CStringArray` from a vector of strings.
    ///
    /// # Arguments
    ///
    /// * `strings` - Vector of strings to convert into C-compatible format
    ///
    /// # Errors
    ///
    /// Returns `CStringArrayError::NulError` if any string contains an interior
    /// null byte. Returns `CStringArrayError::EmptyArray` if the input
    /// vector is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use cstring_array::CStringArray;
    ///
    /// let args = vec!["foo".to_string(), "bar".to_string()];
    /// let array = CStringArray::new(args).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    pub fn new(strings: Vec<String>) -> Result<Self, CStringArrayError> {
        if strings.is_empty() {
            return Err(EmptyArray);
        }

        let cstrings: Vec<CString> = strings
            .into_iter()
            .map(CString::new)
            .collect::<Result<_, _>>()?;

        let mut pointers: Vec<*const c_char> = Vec::with_capacity(cstrings.len() + 1);
        pointers.extend(cstrings.iter().map(|s| s.as_ptr()));
        pointers.push(null());

        Ok(Self {
            strings: cstrings,
            pointers
        })
    }

    /// Creates a new `CStringArray` from a vector of `CString`s (zero-copy).
    ///
    /// This is the most efficient constructor as it takes ownership of
    /// already-allocated `CString` instances without re-allocation.
    ///
    /// # Arguments
    ///
    /// * `strings` - Vector of `CString` instances
    ///
    /// # Errors
    ///
    /// Returns `CStringArrayError::EmptyArray` if the input vector is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use std::ffi::CString;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let cstrings = vec![
    ///     CString::new("hello").unwrap(),
    ///     CString::new("world").unwrap(),
    /// ];
    /// let array = CStringArray::from_cstrings(cstrings).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    pub fn from_cstrings(strings: Vec<CString>) -> Result<Self, CStringArrayError> {
        if strings.is_empty() {
            return Err(EmptyArray);
        }

        let mut pointers: Vec<*const c_char> = Vec::with_capacity(strings.len() + 1);
        pointers.extend(strings.iter().map(|s| s.as_ptr()));
        pointers.push(null());

        Ok(Self {
            strings,
            pointers
        })
    }

    /// Returns a pointer suitable for passing to C functions expecting
    /// `char**`.
    ///
    /// The returned pointer is valid for the lifetime of this `CStringArray`.
    /// The pointer array is null-terminated as required by C conventions.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The pointer is not used after this `CStringArray` is dropped
    /// - The pointer is not used to modify the strings (use `as_mut_ptr` for
    ///   that)
    ///
    /// # Example
    ///
    /// ```
    /// use std::ffi::c_char;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let array = CStringArray::new(vec!["test".to_string()]).unwrap();
    /// let ptr: *const *const c_char = array.as_ptr();
    ///
    /// // Safe to pass to C FFI functions like execve, etc.
    /// ```
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const *const c_char {
        self.pointers.as_ptr()
    }

    /// Returns a mutable pointer suitable for C functions expecting `char**`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The pointer is not used after this `CStringArray` is dropped
    /// - C code does not replace pointers in the array (undefined behavior)
    /// - C code only modifies string contents, not pointer values
    ///
    /// # Example
    ///
    /// ```
    /// use std::ffi::c_char;
    ///
    /// use cstring_array::CStringArray;
    ///
    /// let mut array = CStringArray::new(vec!["test".to_string()]).unwrap();
    /// let ptr: *mut *const c_char = array.as_mut_ptr();
    /// ```
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut *const c_char {
        self.pointers.as_mut_ptr()
    }

    /// Returns the number of strings in the array.
    ///
    /// This count does not include the null terminator.
    ///
    /// # Example
    ///
    /// ```
    /// use cstring_array::CStringArray;
    ///
    /// let array = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    /// assert_eq!(array.len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns `true` if the array contains no strings.
    ///
    /// Note: Due to the constructor constraints, this will always return
    /// `false` for successfully constructed instances, but is provided for
    /// completeness.
    ///
    /// # Example
    ///
    /// ```
    /// use cstring_array::CStringArray;
    ///
    /// let array = CStringArray::new(vec!["x".to_string()]).unwrap();
    /// assert!(!array.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Returns a reference to the underlying `CString` at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the string to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(&CString)` if the index is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use cstring_array::CStringArray;
    ///
    /// let array = CStringArray::new(vec!["first".to_string(), "second".to_string()]).unwrap();
    /// assert_eq!(array.get(0).unwrap().to_str().unwrap(), "first");
    /// assert_eq!(array.get(1).unwrap().to_str().unwrap(), "second");
    /// assert!(array.get(2).is_none());
    /// ```
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&CString> {
        self.strings.get(index)
    }

    /// Returns an iterator over the `CString` references.
    ///
    /// # Example
    ///
    /// ```
    /// use cstring_array::CStringArray;
    ///
    /// let array = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();
    /// let strings: Vec<_> = array.iter().collect();
    /// assert_eq!(strings.len(), 2);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, CString> {
        self.strings.iter()
    }
}

impl Drop for CStringArray {
    fn drop(&mut self) {
        self.pointers.clear();
    }
}

unsafe impl Send for CStringArray {}
unsafe impl Sync for CStringArray {}
