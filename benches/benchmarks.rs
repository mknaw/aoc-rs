use std::fs::read;

use aoc_rs::solutions::y2024::day01;
use criterion::{criterion_group, criterion_main, Criterion};

// This function will contain all benchmarks for day01
fn bench_day01(c: &mut Criterion) {
    // Create a group for day01 benchmarks
    let mut group = c.benchmark_group("day01");
    let s = read("data/y2024/day01.txt").unwrap();

    group.bench_function("part1", |b| b.iter(|| day01::solve_a(&s)));

    group.finish();
}

// You can add more benchmark functions for other days:
// fn bench_day02(c: &mut Criterion) { ... }

criterion_group!(benches, bench_day01);
criterion_main!(benches);
