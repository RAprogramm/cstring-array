// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Environment variables example.
//!
//! This example demonstrates:
//! - Creating CStringArray from environment variables
//! - Formatting strings for C-style environment (KEY=VALUE)
//! - Passing environment to child processes via FFI
//! - Filtering and transforming data

use cstring_array::CStringArray;

fn main() {
    println!("Environment Variables Example\n");

    let env_vars: Vec<String> = std::env::vars()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();

    println!("Total environment variables: {}\n", env_vars.len());

    println!("First 10 environment variables:");
    for (i, var) in env_vars.iter().take(10).enumerate() {
        println!("  [{}] {}", i, var);
    }

    println!("\nCreating CStringArray from environment...");
    let env_array = CStringArray::new(env_vars).expect("Failed to create CStringArray");

    println!("Array created successfully:");
    println!("  Length: {}", env_array.len());
    println!("  Pointer: {:p}", env_array.as_ptr());

    println!("\nSearching for PATH variable:");
    for env in env_array.iter() {
        let env_str = env.to_str().unwrap();
        if env_str.starts_with("PATH=") {
            println!("  Found: {}", env_str);
            break;
        }
    }

    println!("\nFiltering variables with prefix:");
    let prefix = "CARGO_";
    println!("Variables starting with '{}':", prefix);
    for env in env_array.iter() {
        let env_str = env.to_str().unwrap();
        if env_str.starts_with(prefix) {
            println!("  {}", env_str);
        }
    }

    println!("\nCreating filtered environment:");
    let filtered: Vec<String> = std::env::vars()
        .filter(|(key, _)| key.starts_with("CARGO_") || key == "PATH" || key == "HOME")
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();

    let filtered_env = CStringArray::new(filtered).expect("Failed to create filtered array");
    println!("Filtered environment has {} variables:", filtered_env.len());
    for env in filtered_env.iter() {
        println!("  {}", env.to_str().unwrap());
    }

    println!("\nThis array can be passed to execve() or similar C functions.");
}
