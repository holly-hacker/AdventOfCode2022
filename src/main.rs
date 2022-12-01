#![allow(clippy::wildcard_imports)]

include!("lib.rs");

fn main() {
    #[cfg(feature = "day01")]
    day01::Day::execute_gold();
}
