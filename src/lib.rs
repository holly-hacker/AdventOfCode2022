#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::wildcard_imports,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::unreadable_literal
)]

use std::fmt::Debug;

mod utils;

macro_rules! register_days {
    ( $($day_index:literal $type:ident,)* ) => {
        // add `mod`
        $(paste::paste! { #[cfg(feature = "day" $day_index)] pub mod [<day $day_index>]; })*

        /// Run implemenation for all days that are included in the feature set
        pub fn execute_all() {
            $(paste::paste! {
                #[cfg(feature = "day" $day_index)] register_days!(impl $day_index $type);
            })*
        }
    };
    (impl $day_index:literal gold  ) => { paste::paste! { [<day $day_index>]::Day::execute(); }};
    (impl $day_index:literal silver) => { paste::paste! { [<day $day_index>]::Day::execute_silver(); }};
}

// === Register days here! ===
register_days! {
    01 gold,
    02 gold,
    03 gold,
    04 gold,
    05 gold,
    06 gold,
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

pub trait AocDay<T: Debug> {
    const DAY: u32;
    const INPUT_SAMPLE: &'static str;
    const INPUT_REAL: &'static str;

    fn execute_silver() {
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_SAMPLE));
        println!("Day {}, silver (sample): {output:?} ({time:?})", Self::DAY,);
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_REAL));
        println!("Day {}, silver: {output:?} ({time:?})", Self::DAY);

        println!("Day {} has no gold implementation", Self::DAY);
    }

    fn calculate_silver(input: &str) -> T;
}

pub trait AocDayFull<T: Debug>: AocDay<T> {
    fn execute() {
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_SAMPLE));
        println!("Day {}, silver (sample): {output:?} ({time:?})", Self::DAY);
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_REAL));
        println!("Day {}, silver: {output:?} ({time:?})", Self::DAY);

        let (output, time) = run_timed(|| Self::calculate_gold(Self::INPUT_SAMPLE));
        println!("Day {}, gold (sample): {output:?} ({time:?})", Self::DAY);
        let (output, time) = run_timed(|| Self::calculate_gold(Self::INPUT_REAL));
        println!("Day {}, gold: {output:?} ({time:?})", Self::DAY);
    }

    fn calculate_gold(input: &str) -> T;
}
