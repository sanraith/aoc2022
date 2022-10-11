use crate::solution::{SolutionStatic, SolutionType};
use std::collections::HashMap;

// --- Solution modules ---
mod day01;

// --- Re-exports ---
pub use day01::Day01;

// --- Collect all solutions ---
#[derive(Eq, PartialEq, Hash)]
pub struct YearDay(u16, u8);

pub fn get_solution_types() -> HashMap<YearDay, Vec<SolutionType>> {
    let types = [
        Day01::as_type(),
        // ...
    ];

    let mut map = HashMap::new();
    types.into_iter().for_each(|x| {
        let key = YearDay(x.info.year, x.info.day);
        let entry = map.entry(key).or_insert_with(|| Vec::new());
        entry.push(x);
    });

    map
}
