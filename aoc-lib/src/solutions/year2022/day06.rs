use crate::solution::*;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day06;
impl Solution for Day06 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 6, "Tuning Trouble")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let index = find_marker(&ctx, 4);
        Ok(index.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let index = find_marker(&ctx, 14);
        Ok(index.to_string())
    }
}

fn find_marker(ctx: &Context, msg_size: usize) -> usize {
    let (index, _) = ctx
        .input()
        .chars()
        .collect_vec()
        .windows(msg_size)
        .enumerate()
        .filter(|(_, w)| HashSet::<&char>::from_iter(w.iter()).len() == msg_size)
        .next()
        .unwrap();

    return index + msg_size;
}
