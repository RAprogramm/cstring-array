// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Command-line argument handling example.
//!
//! This example demonstrates:
//! - Processing command-line arguments
//! - Creating CStringArray for FFI calls
//! - Simulating a C function that takes char** argv
//! - Real-world usage pattern

use std::ffi::{c_char, c_int, CStr};

use cstring_array::CStringArray;

#[allow(improper_ctypes_definitions)]
extern "C" fn print_args(argc: c_int, argv: *const *const c_char) {
    println!("C function received {} arguments:", argc);
    for i in 0..argc {
        unsafe {
            let arg_ptr = *argv.offset(i as isize);
            if !arg_ptr.is_null() {
                let cstr = CStr::from_ptr(arg_ptr);
                let arg = cstr.to_str().unwrap_or("<invalid utf-8>");
                println!("  argv[{}] = \"{}\"", i, arg);
            }
        }
    }
}

fn main() {
    println!("Command-Line Argument Example\n");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <arg1> <arg2> ...", args[0]);
        println!("\nExample:");
        println!("  cargo run --example command_line -- hello world");
        return;
    }

    println!("Rust received {} arguments:", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("  [{}] {}", i, arg);
    }

    println!("\nPassing to C function via FFI:");
    let array = CStringArray::new(args).expect("Failed to create CStringArray");

    print_args(array.len() as c_int, array.as_ptr());

    println!("\nDemonstrating command parsing:");
    for (i, arg) in array.iter().enumerate() {
        let arg_str = arg.to_str().unwrap();
        match arg_str {
            s if s.starts_with("--") => println!("  Option: {}", s),
            s if s.starts_with('-') => println!("  Flag: {}", s),
            s if i == 0 => println!("  Program: {}", s),
            s => println!("  Argument: {}", s),
        }
    }
}
