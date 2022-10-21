use crate::{solution::SolutionType, solutions, util::YearDay};
use std::collections::HashMap;

pub fn get_solution_types() -> HashMap<YearDay, Vec<SolutionType>> {
    let mut map = HashMap::new();
    solutions::solution_list().into_iter().for_each(|x| {
        let key = YearDay::new(x.info.year, x.info.day);
        map.entry(key).or_insert_with(|| Vec::new()).push(x);
    });

    map
}
