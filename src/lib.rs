#![allow(
    clippy::wildcard_imports,
    clippy::missing_const_for_fn,
    clippy::unreadable_literal
)]

use std::fmt::Display;

mod utils;

macro_rules! register_days {
    ( $($day_index:expr,)* ) => {
        $(
            paste::paste! { #[cfg(feature = "day" $day_index)] pub mod [<day $day_index>]; }
        )*

        pub fn execute_all() {
            $(
                paste::paste! {
                    #[cfg(feature = "day" $day_index)]
                    [<day $day_index>]::Day::execute();
                }
            )*
        }
    }
}

register_days! {
    01,
}

fn run_timed<T, F>(fun: F) -> (T, std::time::Duration)
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
    fn execute() {
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
