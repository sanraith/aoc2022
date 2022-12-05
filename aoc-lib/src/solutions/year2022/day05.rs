use crate::solution::*;
use itertools::Itertools;
use regex::Regex;

#[derive(Default)]
pub struct Day05;
impl Solution for Day05 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 5, "Supply Stacks")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let (mut stacks, steps) = parse_input(&ctx);
        for (count, from, to) in steps {
            for _ in 0..count {
                let item = stacks[from].pop().unwrap();
                stacks[to].push(item);
            }
        }

        let top: String = stacks.into_iter().map(|mut s| s.pop().unwrap()).collect();
        Ok(top)
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let (mut stacks, steps) = parse_input(&ctx);
        for (count, from, to) in steps {
            let target_height = stacks[to].len();
            for _ in 0..count {
                let item = stacks[from].pop().unwrap();
                stacks[to].insert(target_height, item);
            }
        }

        let top: String = stacks.into_iter().map(|mut s| s.pop().unwrap()).collect();
        Ok(top)
    }
}

fn parse_input(ctx: &Context) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = ctx.input();
    let (stack_def, move_def) = input.split_once("\n\n").unwrap();

    let stack_char_width: usize = 4;
    let stack_lines = stack_def.lines().rev().collect_vec();
    let stack_count = (stack_lines[0].len() + 1) / stack_char_width;
    let mut stacks = Vec::with_capacity(stack_count);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }
    for line in stack_lines.iter().skip(1).map(|l| l.chars().collect_vec()) {
        for index in 0..stack_count {
            let c = line[1 + index * stack_char_width];
            if c != ' ' {
                stacks[index].push(c);
            }
        }
    }

    let command_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut steps = Vec::new();
    for line in move_def.lines() {
        let (count, from, to) = command_re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        steps.push((count, from - 1, to - 1));
    }

    (stacks, steps)
}
