// Solution modules
mod day01;

// Re-exports
pub use day01::Day01;

// Collect all solutions
use crate::solution::{SolutionInfo, SolutionType};
pub fn get_all_solutions() -> Vec<SolutionType> {
    vec![
        Day01::as_type(),
        // ...
    ]
}
