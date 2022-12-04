use crate::solution::*;
use itertools::Itertools;

#[derive(Default)]
pub struct Day04;
impl Solution for Day04 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 4, "Camp Cleanup")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let overlap_count = parse_lines(ctx)
            .iter()
            .filter(|(a1, a2, b1, b2)| (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2))
            .count();

        Ok(overlap_count.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let overlap_count = parse_lines(ctx)
            .iter()
            .filter(|(a1, a2, b1, b2)| (a1 <= b1 && b1 <= a2) || (b1 <= a1 && a1 <= b2))
            .count();

        Ok(overlap_count.to_string())
    }
}

fn parse_lines(ctx: &Context) -> Vec<(i32, i32, i32, i32)> {
    ctx.input()
        .lines()
        .filter_map(|l| {
            l.split(",")
                .flat_map(|p| p.split("-").filter_map(|x| x.parse::<i32>().ok()))
                .collect_tuple()
        })
        .collect_vec()
}
