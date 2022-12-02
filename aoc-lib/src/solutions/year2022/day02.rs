use crate::solution::*;
use itertools::Itertools;

const CHARCODE_A: u32 = 65;
const CHARCODE_X: u32 = 88;

/// [opponents_play] = my_play_to_win
const IS_WIN: [u32; 3] = [1, 2, 0];

/// [target_result] = my_diff_to_result
const DIFF_TO_RESULT: [u32; 3] = [2, 0, 1];

#[derive(Default)]
pub struct Day02;
impl Solution for Day02 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 2, "Rock Paper Scissors")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let points = parse_rounds(ctx)
            .into_iter()
            .fold(0, |acc, (a, b)| acc + b + 1 + calc_points(a, b));

        Ok(points.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let points = parse_rounds(ctx)
            .into_iter()
            .map(|(a, b)| (a, (a + DIFF_TO_RESULT[b as usize]) % 3))
            .fold(0, |acc, (a, b)| acc + b + 1 + calc_points(a, b));

        Ok(points.to_string())
    }
}

/// Parse rounds into (a, b) pairs, where
/// [A, B, C] => [0, 1, 2] and
/// [X, Y, Z] => [0, 1, 2]
fn parse_rounds(ctx: &Context) -> Vec<(u32, u32)> {
    ctx.input()
        .lines()
        .filter_map(|l| {
            l.split(" ")
                .map(|l| l.chars().next().unwrap() as u32)
                .collect_tuple::<(_, _)>()
        })
        .map(|(a, b)| (a - CHARCODE_A, b - CHARCODE_X))
        .collect_vec()
}

fn calc_points(opponent: u32, me: u32) -> u32 {
    match (opponent, me) {
        _ if opponent == me => 3,
        _ if IS_WIN[opponent as usize] == me => 6,
        _ => 0,
    }
}
