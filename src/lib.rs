#![allow(clippy::wildcard_imports)]

use std::fmt::Display;

mod utils;

#[cfg(feature = "day01")]
pub mod day01;

/// Executes some code and records the time it took to run
pub fn run_timed<T, F>(fun: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let now = std::time::Instant::now();
    let ret = fun();
    let elapsed = now.elapsed();
    (ret, elapsed)
}

pub trait AocDay<T: Display> {
    const DAY: u32;
    const INPUT_SAMPLE: &'static str;
    const INPUT_REAL: &'static str;

    fn execute_silver() {
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_SAMPLE));
        println!(
            "Day {}, silver (sample): {} ({:?})",
            Self::DAY,
            output,
            time
        );
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_REAL));
        println!("Day {}, silver: {} ({:?})", Self::DAY, output, time);

        println!("Day {}, gold (sample): skipped", Self::DAY);
        println!("Day {}, gold: skipped", Self::DAY);
    }

    fn calculate_silver(input: &str) -> T;
}

pub trait AocDayFull<T: Display>: AocDay<T> {
    fn execute_gold() {
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_SAMPLE));
        println!(
            "Day {}, silver (sample): {} ({:?})",
            Self::DAY,
            output,
            time
        );
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_REAL));
        println!("Day {}, silver: {} ({:?})", Self::DAY, output, time);

        let (output, time) = run_timed(|| Self::calculate_gold(Self::INPUT_SAMPLE));
        println!("Day {}, gold (sample): {} ({:?})", Self::DAY, output, time);
        let (output, time) = run_timed(|| Self::calculate_gold(Self::INPUT_REAL));
        println!("Day {}, gold: {} ({:?})", Self::DAY, output, time);
    }

    fn calculate_gold(input: &str) -> T;
}
