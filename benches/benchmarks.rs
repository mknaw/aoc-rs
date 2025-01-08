use std::fs::read;

use aoc_rs::solutions::y2024::day01;
use criterion::{criterion_group, criterion_main, Criterion};
use seq_macro::seq;

seq!(DAY in 01..=31 {
    fn bench_day~DAY(c: &mut Criterion) {
        let mut group = c.benchmark_group(format!("day{:02}", DAY));
        let s = read(format!("data/y2024/day{:02}.txt", DAY)).unwrap();

        group.bench_function("part_a", |b| b.iter(|| day01::solve_a(&s)));

        group.finish();
    }

});

criterion_group!(benches, bench_day01, bench_day04);

criterion_main!(benches);
