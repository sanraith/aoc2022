use crate::solution::*;
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use std::collections::{HashMap, HashSet};

const TILE_ELF: char = '#';
const TILE_EMPTY: char = '.';
const DIRECTION_COUNT: usize = 12;
const CARD_DIR_COUNT: usize = 4;
const SUB_DIR_COUNT: usize = 3;
const DIRECTIONS: [Point; 12] = [
    Point { x: 0, y: -1 }, // North
    Point { x: 1, y: -1 },
    Point { x: -1, y: -1 },
    Point { x: 0, y: 1 }, // South
    Point { x: 1, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: 0 }, // West
    Point { x: -1, y: -1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: 0 }, // East
    Point { x: 1, y: -1 },
    Point { x: 1, y: 1 },
];

#[derive(Default)]
pub struct Day23;
impl Solution for Day23 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 23, "Unstable Diffusion")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let mut map = parse_map(ctx);
        for round in 0..10 {
            (map, ..) = execute_round(map, round);
        }

        let (min, max) = map.bounds();
        let empty_count = itertools::iproduct!(min.x..=max.x, min.y..=max.y)
            .filter(|(x, y)| !map.tiles.contains(&Point::new(*x, *y)))
            .count();

        Ok(empty_count.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let mut map = parse_map(ctx);
        let mut move_count = 1;
        let mut round = 0;
        while move_count > 0 {
            ctx.progress(round as f32 / 1000.0);
            (map, move_count) = execute_round(map, round);
            round += 1;
        }

        Ok(round.to_string())
    }
}

fn execute_round(prev: Map, round_idx: usize) -> (Map, i32) {
    let start_dir = (round_idx * SUB_DIR_COUNT) % DIRECTION_COUNT;

    let mut next = Map::default();
    let mut proposed_moves: HashMap<Point, Vec<Point>> = HashMap::new();
    'elves: for elf in &prev.tiles {
        let all_neighbors_empty = DIRECTIONS
            .iter()
            .all(|dir| prev.tiles.get(&(*elf + *dir)).is_none());
        if all_neighbors_empty {
            next.tiles.insert(*elf);
            continue;
        }

        for cardinal_idx in 0..CARD_DIR_COUNT {
            let start_idx = (start_dir + cardinal_idx * SUB_DIR_COUNT) % DIRECTION_COUNT;
            let is_direction_empty = (start_idx..start_idx + SUB_DIR_COUNT)
                .all(|i| prev.tiles.get(&(*elf + DIRECTIONS[i])).is_none());
            if is_direction_empty {
                let target = *elf + DIRECTIONS[start_idx];
                proposed_moves
                    .entry(target)
                    .or_insert_with(|| Vec::new())
                    .push(*elf);
                continue 'elves;
            }
        }

        // cannot move
        next.tiles.insert(*elf);
    }

    let mut move_count = 0;
    for (target, elves) in proposed_moves {
        match elves.len() {
            1 => {
                next.tiles.insert(target);
                move_count += 1;
            }
            _ => elves.into_iter().for_each(|p| _ = next.tiles.insert(p)),
        };
    }

    (next, move_count)
}

fn parse_map(ctx: &Context) -> Map {
    let mut tiles = HashSet::new();
    for (y, line) in ctx.input().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == TILE_ELF {
                tiles.insert(Point::new(x as i32, y as i32));
            }
        }
    }

    Map { tiles }
}

#[derive(Debug, Default)]
struct Map {
    tiles: HashSet<Point>,
}
impl Map {
    fn bounds(&self) -> (Point, Point) {
        let mut min = Point::new(i32::MAX, i32::MAX);
        let mut max = Point::new(i32::MIN, i32::MIN);
        for &p in &self.tiles {
            min = min.min_parts(p);
            max = max.max_parts(p);
        }

        (min, max)
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (min, max) = self.bounds();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                match self.tiles.contains(&Point::new(x, y)) {
                    true => print!("{TILE_ELF}"),
                    false => print!("{TILE_EMPTY}"),
                }
            }
            println!();
        }
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn min_parts(&self, b: Point) -> Point {
        Point::new(self.x.min(b.x), self.y.min(b.y))
    }

    pub fn max_parts(&self, b: Point) -> Point {
        Point::new(self.x.max(b.x), self.y.max(b.y))
    }
}
