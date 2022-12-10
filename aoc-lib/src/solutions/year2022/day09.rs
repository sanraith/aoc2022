use crate::solution::*;
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Day09;
impl Solution for Day09 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 9, "Rope Bridge")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let visited = simulate_rope(2, ctx);
        Ok(visited.len().to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let visited = simulate_rope(10, ctx);
        Ok(visited.len().to_string())
    }
}

fn simulate_rope(rope_length: usize, ctx: &Context) -> HashSet<Point> {
    let steps = parse_steps(ctx);
    let mut rope = (0..rope_length).map(|_| Point::new(0, 0)).collect_vec();
    let mut visited: HashSet<Point> = HashSet::from_iter([Point::new(0, 0)]);

    for (dir, count) in steps {
        for _ in 0..count {
            rope[0] += dir;
            for (prev_idx, current_idx) in (0..rope_length).tuple_windows() {
                let head = rope[prev_idx];
                let tail = &mut rope[current_idx];
                let diff = head - *tail;
                let is_touching = diff.x.abs() <= 1 && diff.y.abs() <= 1;
                if !is_touching {
                    *tail += Point::new(diff.x.signum(), diff.y.signum());
                    if current_idx == rope_length - 1 {
                        visited.insert(*tail);
                    }
                }
            }
        }
    }
    visited
}

fn parse_steps(ctx: &Context) -> Vec<(Point, i32)> {
    let directions: HashMap<char, Point> = HashMap::from_iter([
        ('U', Point::new(0, -1)),
        ('D', Point::new(0, 1)),
        ('L', Point::new(-1, 0)),
        ('R', Point::new(1, 0)),
    ]);

    let steps = ctx
        .input()
        .lines()
        .map(|line| {
            let (dir_str, count_str) = line.split_once(" ").unwrap();
            let dir = *directions.get(&dir_str.chars().next().unwrap()).unwrap();
            let count = count_str.parse().unwrap();
            (dir, count)
        })
        .collect_vec();
    steps
}

#[derive(Copy, Clone, Debug, Constructor, Add, Sub, AddAssign, SubAssign, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
