// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::{convert::TryFrom, ffi::CString};

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use cstring_array::CStringArray;

fn bench_new_from_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("new_from_strings");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let strings: Vec<String> = (0..size).map(|i| format!("string_{}", i)).collect();

            b.iter(|| {
                let array = CStringArray::new(black_box(strings.clone())).unwrap();
                black_box(array);
            });
        });
    }

    group.finish();
}

fn bench_from_cstrings_zero_copy(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_cstrings_zero_copy");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let cstrings: Vec<CString> = (0..size)
                .map(|i| CString::new(format!("string_{}", i)).unwrap())
                .collect();

            b.iter(|| {
                let array = CStringArray::from_cstrings(black_box(cstrings.clone())).unwrap();
                black_box(array);
            });
        });
    }

    group.finish();
}

fn bench_as_ptr(c: &mut Criterion) {
    let array = CStringArray::new(
        (0..100)
            .map(|i| format!("string_{}", i))
            .collect::<Vec<_>>()
    )
    .unwrap();

    c.bench_function("as_ptr", |b| {
        b.iter(|| {
            let ptr = black_box(&array).as_ptr();
            black_box(ptr);
        });
    });
}

fn bench_get(c: &mut Criterion) {
    let array = CStringArray::new(
        (0..1000)
            .map(|i| format!("string_{}", i))
            .collect::<Vec<_>>()
    )
    .unwrap();

    c.bench_function("get", |b| {
        b.iter(|| {
            let item = black_box(&array).get(black_box(500));
            black_box(item);
        });
    });
}

fn bench_iter(c: &mut Criterion) {
    let array = CStringArray::new(
        (0..1000)
            .map(|i| format!("string_{}", i))
            .collect::<Vec<_>>()
    )
    .unwrap();

    c.bench_function("iter", |b| {
        b.iter(|| {
            for item in black_box(&array).iter() {
                black_box(item);
            }
        });
    });
}

fn bench_try_from_vec_str(c: &mut Criterion) {
    let strings: Vec<&str> = (0..100).map(|_| "benchmark_string").collect();

    c.bench_function("try_from_vec_str", |b| {
        b.iter(|| {
            let array = CStringArray::try_from(black_box(strings.clone())).unwrap();
            black_box(array);
        });
    });
}

fn bench_new_from_iter(c: &mut Criterion) {
    c.bench_function("new_from_iter", |b| {
        b.iter(|| {
            let strings: Vec<String> = (0..100).map(|i| format!("string_{}", i)).collect();
            let array = CStringArray::new(black_box(strings)).unwrap();
            black_box(array);
        });
    });
}

fn bench_construction_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction_comparison");

    let strings: Vec<String> = (0..100).map(|i| format!("string_{}", i)).collect();
    let str_refs: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

    group.bench_function("from_vec_string", |b| {
        b.iter(|| {
            let array = CStringArray::new(black_box(strings.clone())).unwrap();
            black_box(array);
        });
    });

    group.bench_function("try_from_vec_str", |b| {
        b.iter(|| {
            let array = CStringArray::try_from(black_box(str_refs.clone())).unwrap();
            black_box(array);
        });
    });

    group.bench_function("from_vec_new", |b| {
        b.iter(|| {
            let array = CStringArray::new(black_box(strings.clone())).unwrap();
            black_box(array);
        });
    });

    group.finish();
}

fn bench_large_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_strings");

    for string_size in [100, 1000, 10000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(string_size),
            string_size,
            |b, &size| {
                let large_string = "a".repeat(size);
                let strings = vec![large_string; 10];

                b.iter(|| {
                    let array = CStringArray::new(black_box(strings.clone())).unwrap();
                    black_box(array);
                });
            }
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_new_from_strings,
    bench_from_cstrings_zero_copy,
    bench_as_ptr,
    bench_get,
    bench_iter,
    bench_try_from_vec_str,
    bench_new_from_iter,
    bench_construction_comparison,
    bench_large_strings,
);

criterion_main!(benches);
