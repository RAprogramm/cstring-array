// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Basic usage example of CStringArray.
//!
//! This example demonstrates:
//! - Creating a CStringArray from Vec<String>
//! - Converting to C-compatible char** pointer
//! - Safe iteration over elements
//! - Automatic memory cleanup

use cstring_array::CStringArray;

fn main() {
    println!("Basic CStringArray Usage Example\n");

    let args = vec![
        "my-program".to_string(),
        "--verbose".to_string(),
        "input.txt".to_string(),
        "output.txt".to_string(),
    ];

    println!("Creating CStringArray from {} strings:", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("  [{}] {}", i, arg);
    }

    let array = CStringArray::new(args).expect("Failed to create CStringArray");

    println!("\nArray properties:");
    println!("  Length: {}", array.len());
    println!("  Empty: {}", array.is_empty());
    println!("  Pointer: {:p}", array.as_ptr());

    println!("\nIterating over elements:");
    for (i, cstr) in array.iter().enumerate() {
        let s = cstr.to_str().unwrap_or("<invalid utf-8>");
        println!("  [{}] {}", i, s);
    }

    println!("\nAccessing individual elements:");
    if let Some(first) = array.get(0) {
        println!("  First: {}", first.to_str().unwrap());
    }
    if let Some(last) = array.get(array.len() - 1) {
        println!("  Last: {}", last.to_str().unwrap());
    }

    println!("\nArray will be automatically freed when it goes out of scope.");
}
