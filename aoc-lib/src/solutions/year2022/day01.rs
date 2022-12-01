use crate::solution::*;
use itertools::Itertools;

#[derive(Default)]
pub struct Day01;
impl Solution for Day01 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 1, "Calorie Counting")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let max = ctx
            .input()
            .split("\n\n")
            .map(|p| p.lines().filter_map(|x| x.parse::<i32>().ok()).sum::<i32>())
            .max()
            .unwrap_or(0);

        Ok(max.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let max = ctx
            .input()
            .split("\n\n")
            .map(|p| p.lines().filter_map(|x| x.parse::<i32>().ok()).sum::<i32>())
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<i32>();

        Ok(max.to_string())
    }
}
