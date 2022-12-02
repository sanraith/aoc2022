// --- This file was auto-generated by build.rs ---

#[allow(unused_imports)]
use crate::solution::{SolutionStatic, SolutionType};
use crate::util::YearDay;
use std::collections::HashMap;

// Module definitions
pub mod day01;
pub mod day02;

// Re-exports
pub use day01::Day01;
pub use day02::Day02;

// List of solutions
#[allow(unused_mut)]
pub fn create_list() -> Vec<SolutionType> {
    let mut list = vec![
        Day01::as_type(),
        Day02::as_type(),
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
