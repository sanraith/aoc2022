use crate::{
    solution::{SolutionStatic, SolutionType},
    solutions::*,
    util::YearDay,
};
use std::collections::HashMap;

pub fn get_solution_types() -> HashMap<YearDay, Vec<SolutionType>> {
    let types = [
        Day01::as_type(),
        // ...
    ];

    let mut map = HashMap::new();
    types.into_iter().for_each(|x| {
        let key = YearDay::new(x.info.year, x.info.day);
        let entry = map.entry(key).or_insert_with(|| Vec::new());
        entry.push(x);
    });

    map
}
