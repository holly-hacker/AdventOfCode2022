use aoc2022::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 01 silver (sample)", |b| {
        b.iter(|| day01::Day::calculate_silver(black_box(day01::Day::INPUT_SAMPLE)))
    });
    c.bench_function("Day 01 silver (real)", |b| {
        b.iter(|| day01::Day::calculate_silver(black_box(day01::Day::INPUT_REAL)))
    });
    c.bench_function("Day 01 gold (sample)", |b| {
        b.iter(|| day01::Day::calculate_gold(black_box(day01::Day::INPUT_SAMPLE)))
    });
    c.bench_function("Day 01 gold (real)", |b| {
        b.iter(|| day01::Day::calculate_gold(black_box(day01::Day::INPUT_REAL)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
