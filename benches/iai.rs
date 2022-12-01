use aoc2022::*;
use iai::black_box;

pub fn day01_silver_sample() {
    day01::Day::calculate_silver(black_box(day01::Day::INPUT_SAMPLE));
}

pub fn day01_silver_real() {
    day01::Day::calculate_silver(black_box(day01::Day::INPUT_REAL));
}

pub fn day01_gold_sample() {
    day01::Day::calculate_gold(black_box(day01::Day::INPUT_SAMPLE));
}

pub fn day01_gold_real() {
    day01::Day::calculate_gold(black_box(day01::Day::INPUT_REAL));
}

iai::main!(
    day01_silver_sample,
    day01_silver_real,
    day01_gold_sample,
    day01_gold_real,
);
