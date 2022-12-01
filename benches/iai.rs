// Based on iai 0.1.1's `main` macro.
macro_rules! gen_benchmarks {
    ( $($day_index:expr,)* ) => {
        paste::paste! {
            mod iai_wrappers {
                use aoc2022::*;
                use iai::black_box;
                $(
                    #[cfg(feature = "day" $day_index)]
                    pub fn [< day_ $day_index _silver_sample >]() {
                        let _ = black_box(aoc2022::[<day $day_index>]::Day::calculate_silver(black_box([<day $day_index>]::Day::INPUT_SAMPLE)));
                    }

                    #[cfg(feature = "day" $day_index)]
                    pub fn [< day_ $day_index _silver_real >]() {
                        let _ = black_box(aoc2022::[<day $day_index>]::Day::calculate_silver(black_box([<day $day_index>]::Day::INPUT_REAL)));
                    }

                    #[cfg(feature = "day" $day_index)]
                    pub fn [< day_ $day_index _gold_sample >]() {
                        let _ = black_box(aoc2022::[<day $day_index>]::Day::calculate_gold(black_box([<day $day_index>]::Day::INPUT_SAMPLE)));
                    }

                    #[cfg(feature = "day" $day_index)]
                    pub fn [< day_ $day_index _gold_real >]() {
                        let _ = black_box(aoc2022::[<day $day_index>]::Day::calculate_gold(black_box([<day $day_index>]::Day::INPUT_REAL)));
                    }
                )*
            }
        }

        fn main() {
            let benchmarks : &[&(&'static str, fn())]= &[
                $(
                    paste::paste!(#[cfg(feature = "day" $day_index)] &(stringify!([< day_ $day_index _silver_sample >]), iai_wrappers::[< day_ $day_index _silver_sample >])),
                    paste::paste!(#[cfg(feature = "day" $day_index)] &(stringify!([< day_ $day_index _silver_real >]), iai_wrappers::[< day_ $day_index _silver_real >])),
                    paste::paste!(#[cfg(feature = "day" $day_index)] &(stringify!([< day_ $day_index _gold_sample >]), iai_wrappers::[< day_ $day_index _gold_sample >])),
                    paste::paste!(#[cfg(feature = "day" $day_index)] &(stringify!([< day_ $day_index _gold_real >]), iai_wrappers::[< day_ $day_index _gold_real >])),
                )*

            ];

            iai::runner(benchmarks);
        }
    }
}

gen_benchmarks! {
    01,
}
