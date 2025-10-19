// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Zero-copy construction example.
//!
//! This example demonstrates:
//! - Creating CStringArray from Vec<CString> (zero-copy)
//! - Performance benefits of zero-copy construction
//! - When to use from_cstrings() vs new()
//! - Memory efficiency

use std::ffi::CString;

use cstring_array::CStringArray;

fn main() {
    println!("Zero-Copy Construction Example\n");

    println!("Method 1: Regular construction (allocates CString internally)");
    let strings = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ];
    let array1 = CStringArray::new(strings).expect("Failed to create array");
    println!("  Created array with {} elements", array1.len());

    println!("\nMethod 2: Zero-copy construction (no re-allocation)");
    let cstrings = vec![
        CString::new("first").unwrap(),
        CString::new("second").unwrap(),
        CString::new("third").unwrap(),
    ];

    println!("  Pre-allocated {} CStrings", cstrings.len());
    let array2 = CStringArray::from_cstrings(cstrings).expect("Failed to create array");
    println!("  Created array with {} elements (zero-copy)", array2.len());

    println!("\nPerformance comparison:");
    println!("  Regular: String -> CString (internal allocation)");
    println!("  Zero-copy: Vec<CString> -> Array (move ownership)");

    println!("\nUse zero-copy when:");
    println!("  - You already have Vec<CString>");
    println!("  - Performance is critical");
    println!("  - Working with large arrays");
    println!("  - Avoiding redundant allocations");

    println!("\nDemonstrating with larger dataset:");
    let large_cstrings: Vec<CString> = (0..1000)
        .map(|i| CString::new(format!("string_{}", i)).unwrap())
        .collect();

    println!("  Created {} CStrings", large_cstrings.len());
    let large_array = CStringArray::from_cstrings(large_cstrings).expect("Failed to create array");
    println!("  Zero-copy array length: {}", large_array.len());
    println!(
        "  First element: {}",
        large_array.get(0).unwrap().to_str().unwrap()
    );
    println!(
        "  Last element: {}",
        large_array
            .get(large_array.len() - 1)
            .unwrap()
            .to_str()
            .unwrap()
    );

    println!("\nMemory is automatically freed when array goes out of scope.");
}
