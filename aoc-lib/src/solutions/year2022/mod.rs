// --- This file was auto-generated by build.rs ---

#[allow(unused_imports)]
use crate::solution::{SolutionStatic, SolutionType};
use crate::util::YearDay;
use std::collections::HashMap;

// Module definitions
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

// Re-exports
pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;
pub use day07::Day07;

// List of solutions
#[allow(unused_mut)]
pub fn create_list() -> Vec<SolutionType> {
    let mut list = vec![
        Day01::as_type(),
        Day02::as_type(),
        Day03::as_type(),
        Day04::as_type(),
        Day05::as_type(),
        Day06::as_type(),
        Day07::as_type(),
    ];

    list
}

// Map of solutions
pub fn create_map() -> HashMap<YearDay, Vec<SolutionType>> {
    let mut map = HashMap::new();
    create_list().into_iter().for_each(|x| {
        let key = YearDay::new(x.info.year, x.info.day);
        map.entry(key).or_insert_with(|| Vec::new()).push(x);
    });

    map
}
