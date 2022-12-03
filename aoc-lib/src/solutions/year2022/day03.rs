use crate::{
    solution::*,
    util::{GenericResult, MsgError},
};
use itertools::Itertools;

#[derive(Default)]
pub struct Day03;
impl Solution for Day03 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 3, "Rucksack Reorganization")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let mut sum = 0;
        for rucksack in ctx.input().lines() {
            let mut common = None;
            let (a, b) = rucksack.split_at(rucksack.len() / 2);
            for letter in a.chars() {
                if b.contains(letter) {
                    common = Some(letter);
                    break;
                }
            }

            let common = common.ok_or(MsgError("No common items"))?;
            sum += get_priority(common)?;
        }

        Ok(sum.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let mut sum = 0;
        for group in &ctx.input().lines().into_iter().chunks(3) {
            let group = group.collect_vec();
            let mut common = None;
            for letter in group[0].chars() {
                if group.iter().skip(1).all(|x| x.contains(letter)) {
                    common = Some(letter);
                    break;
                }
            }

            let common = common.ok_or(MsgError("No common items"))?;
            sum += get_priority(common)?;
        }

        Ok(sum.to_string())
    }
}

fn get_priority(item: char) -> GenericResult<u32> {
    match item {
        'a'..='z' => Ok(item as u32 - 96),
        'A'..='Z' => Ok(item as u32 - 38),
        _ => Err(MsgError("Unknown priority"))?,
    }
}
