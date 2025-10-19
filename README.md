# cstring-array

[![Crates.io](https://img.shields.io/crates/v/cstring-array.svg)](https://crates.io/crates/cstring-array)
[![Downloads](https://img.shields.io/crates/d/cstring-array.svg)](https://crates.io/crates/cstring-array)
[![Documentation](https://docs.rs/cstring-array/badge.svg)](https://docs.rs/cstring-array)
[![Lib.rs](https://img.shields.io/badge/lib.rs-cstring--array-blue)](https://lib.rs/crates/cstring-array)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[![CI](https://github.com/RAprogramm/cstring-array/workflows/CI/badge.svg)](https://github.com/RAprogramm/cstring-array/actions)
[![codecov](https://codecov.io/gh/RAprogramm/cstring-array/graph/badge.svg?token=7qIC3Impoa)](https://codecov.io/gh/RAprogramm/cstring-array)
[![REUSE status](https://api.reuse.software/badge/github.com/RAprogramm/cstring-array)](https://api.reuse.software/info/github.com/RAprogramm/cstring-array)

[![Rust Version](https://img.shields.io/badge/rust-1.90%2B-blue.svg)](https://www.rust-lang.org)
[![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/cstring-array?branch=main)](https://hitsofcode.com/view/github/RAprogramm/cstring-array?branch=main)

**Safe, zero-copy wrapper for passing string arrays to C FFI (`char**`)**

This crate provides `CStringArray`, a safe abstraction over C's null-terminated string arrays, commonly used for command-line arguments (`argv`) and similar purposes.

## Features

- **Memory-safe**: RAII-based lifetime management prevents dangling pointers
- **Zero-copy**: When constructed from `Vec<CString>`, no re-allocation occurs
- **C-compatible**: Produces valid `char**` pointers with null termination
- **Ergonomic**: Multiple constructors and trait implementations for easy usage
- **Well-tested**: 98.5%+ test coverage for reliability
- **Minimal dependencies**: Pure Rust with no external dependencies
- **Cross-platform**: Works on Linux, macOS, Windows, and more

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
cstring-array = "0.1"
```

## Usage

### Basic Example

```rust
use cstring_array::CStringArray;
use std::ffi::c_char;

let args = vec![
    "program".to_string(),
    "--verbose".to_string(),
    "file.txt".to_string(),
];
let array = CStringArray::new(args).unwrap();

// Safe to pass to C FFI functions expecting char**
let ptr: *const *const c_char = array.as_ptr();
assert_eq!(array.len(), 3);
```

### Construction Methods

```rust
use cstring_array::CStringArray;
use std::ffi::CString;
use std::convert::TryFrom;

// From Vec<String>
let arr1 = CStringArray::new(vec!["foo".to_string(), "bar".to_string()]).unwrap();

// From Vec<CString> (zero-copy)
let cstrings = vec![CString::new("foo").unwrap(), CString::new("bar").unwrap()];
let arr2 = CStringArray::from_cstrings(cstrings).unwrap();

// Using TryFrom with Vec<&str>
let arr3 = CStringArray::try_from(vec!["foo", "bar"]).unwrap();

// Using TryFrom with arrays
let arr4 = CStringArray::try_from(["foo", "bar"]).unwrap();

// Using FromIterator (collect)
let arr5: CStringArray = vec!["a", "b", "c"].into_iter().map(String::from).collect();
```

### Ergonomic API with Standard Traits

`CStringArray` implements standard Rust traits for idiomatic usage:

```rust
use cstring_array::CStringArray;
use std::collections::HashMap;

let arr1 = CStringArray::new(vec!["a".to_string(), "b".to_string()]).unwrap();

// Clone - independent copies
let arr2 = arr1.clone();
assert_eq!(arr1, arr2);

// Equality comparison
assert_eq!(arr1, arr2);

// Hash - use in HashMap/HashSet
let mut map = HashMap::new();
map.insert(arr1.clone(), "value");

// Array indexing
assert_eq!(arr1[0].to_str().unwrap(), "a");
assert_eq!(arr1[1].to_str().unwrap(), "b");

// For loop iteration (borrowed)
for s in &arr1 {
    println!("{}", s.to_str().unwrap());
}

// For loop iteration (owned - consumes array)
for s in arr2 {
    println!("{}", s.to_str().unwrap());
}

// Functional programming with iterators
let filtered: CStringArray = vec!["a", "bb", "ccc"]
    .into_iter()
    .filter(|s| s.len() > 1)
    .map(String::from)
    .collect();
```

**Implemented traits:**
- `Clone` - Deep copy with independent memory
- `PartialEq`, `Eq` - Equality comparison
- `Hash` - Use in `HashMap` and `HashSet`
- `FromIterator<String>` - Collect from `String` iterators
- `FromIterator<CString>` - Collect from `CString` iterators
- `IntoIterator` - Owned and borrowed iteration
- `Index<usize>` - Array-style indexing `arr[0]`
- `AsRef<[CString]>` - Generic slice conversion

### Real-World Example: Calling C Function

```rust
use cstring_array::CStringArray;
use std::ffi::c_char;

extern "C" {
    fn execve(
        pathname: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> i32;
}

fn execute_program(path: &str, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let argv = CStringArray::new(args)?;
    let envp = CStringArray::new(vec![])?;  // Empty environment

    unsafe {
        execve(
            CString::new(path)?.as_ptr(),
            argv.as_ptr(),
            envp.as_ptr(),
        );
    }

    Ok(())
}
```

### Error Handling

```rust
use cstring_array::{CStringArray, CStringArrayError};

// Interior null bytes are detected
let result = CStringArray::new(vec!["hello\0world".to_string()]);
assert!(matches!(result, Err(CStringArrayError::NulError(_))));

// Empty arrays are not allowed
let result = CStringArray::new(vec![]);
assert!(matches!(result, Err(CStringArrayError::EmptyArray)));
```

## Safety Considerations

The pointer returned by `CStringArray::as_ptr()` is only valid for the lifetime of the `CStringArray`. Ensure the array outlives any C code using the pointer:

```rust
use cstring_array::CStringArray;
use std::ffi::c_char;

fn call_c_function(argv: *const *const c_char, argc: i32) {
    // ... FFI call ...
}

let array = CStringArray::new(vec!["arg1".to_string(), "arg2".to_string()]).unwrap();
let ptr = array.as_ptr();
call_c_function(ptr, array.len() as i32);
// array must not be dropped before call_c_function returns
```

## Performance

`CStringArray` is designed for zero-cost abstractions:

- **Zero-copy** when constructed from `Vec<CString>`
- **No re-allocation** of strings, only pointer array management
- **RAII cleanup** without manual memory management

### Benchmark Results

<details>
<summary><b>Performance Metrics</b></summary>

<!-- BENCHMARK_RESULTS_START -->
*Last updated: 2025-10-19 02:42:47 UTC*

#### Operations

| Benchmark | Time | Std Dev |
|-----------|------|---------|
| As Ptr | 0 ns | ±0 ns |
| Get | 0 ns | ±0 ns |
| Iter | 317 ns | ±1 ns |
| Try From Vec Str | 4.89 μs | ±15 ns |
| New From Iter | 7.52 μs | ±25 ns |

#### Construction Comparison

| Benchmark | Time | Std Dev |
|-----------|------|---------|
| Construction Comparison/From Vec New | 5.13 μs | ±47 ns |
| Construction Comparison/From Vec String | 5.16 μs | ±17 ns |
| Construction Comparison/Try From Vec Str | 5.29 μs | ±25 ns |

#### From Cstrings Zero Copy

| Benchmark | Time | Std Dev |
|-----------|------|---------|
| From Cstrings Zero Copy/10 | 205 ns | ±0 ns |
| From Cstrings Zero Copy/100 | 3.66 μs | ±11 ns |
| From Cstrings Zero Copy/1000 | 35.30 μs | ±111 ns |

#### Large Strings

| Benchmark | Time | Std Dev |
|-----------|------|---------|
| Large Strings/100 | 368 ns | ±1 ns |
| Large Strings/1000 | 1.56 μs | ±24 ns |
| Large Strings/10000 | 8.01 μs | ±47 ns |

#### New From Strings

| Benchmark | Time | Std Dev |
|-----------|------|---------|
| New From Strings/10 | 330 ns | ±5 ns |
| New From Strings/100 | 4.76 μs | ±66 ns |
| New From Strings/1000 | 48.54 μs | ±283 ns |
<!-- BENCHMARK_RESULTS_END -->

</details>

<details>
<summary><b>Coverage Visualization</b></summary>

### Sunburst Chart
The inner-most circle is the entire project, moving away from the center are folders then, finally, a single file. The size and color of each slice represents the number of statements and the coverage, respectively.

[![Sunburst](https://codecov.io/gh/RAprogramm/cstring-array/graphs/sunburst.svg?token=7qIC3Impoa)](https://codecov.io/gh/RAprogramm/cstring-array)

### Grid Chart
Each block represents a single file in the project. The size and color of each block is represented by the number of statements and the coverage, respectively.

[![Grid](https://codecov.io/gh/RAprogramm/cstring-array/graphs/tree.svg?token=7qIC3Impoa)](https://codecov.io/gh/RAprogramm/cstring-array)

### Icicle Chart
The top section represents the entire project, proceeding with folders and finally individual files. The size and color of each slice represents the number of statements and the coverage, respectively.

[![Icicle](https://codecov.io/gh/RAprogramm/cstring-array/graphs/icicle.svg?token=7qIC3Impoa)](https://codecov.io/gh/RAprogramm/cstring-array)

</details>

## Minimum Supported Rust Version (MSRV)

This crate requires Rust 1.90 or later.

## Related Projects

- [`std::ffi::CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html) - Standard library C string type
- [`std::ffi::CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html) - Borrowed C string slice
