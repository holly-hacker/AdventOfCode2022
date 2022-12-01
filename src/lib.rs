use std::fmt::Display;

#[cfg(feature = "day01")]
pub mod day01;

pub trait AocDay<T: Display> {
    const DAY: u32;
    const INPUT_SAMPLE: &'static str;
    const INPUT_REAL: &'static str;

    fn execute_silver() {
        println!("Executing silver-only");

        println!(
            "Day {}, silver (sample): {}",
            Self::DAY,
            Self::calculate_silver(Self::INPUT_SAMPLE)
        );
        println!(
            "Day {}, silver: {}",
            Self::DAY,
            Self::calculate_silver(Self::INPUT_REAL)
        );
    }

    fn calculate_silver(input: &str) -> T;
}

pub trait AocDayFull<T: Display>: AocDay<T> {
    fn execute_gold() {
        println!("Executing full");

        println!(
            "Day {}, silver (sample): {}",
            Self::DAY,
            Self::calculate_silver(Self::INPUT_SAMPLE)
        );
        println!(
            "Day {}, silver: {}",
            Self::DAY,
            Self::calculate_silver(Self::INPUT_REAL)
        );

        println!(
            "Day {}, gold (sample): {}",
            Self::DAY,
            Self::calculate_gold(Self::INPUT_SAMPLE)
        );
        println!(
            "Day {}, gold: {}",
            Self::DAY,
            Self::calculate_gold(Self::INPUT_REAL)
        );
    }

    fn calculate_gold(input: &str) -> T;
}
