use std::collections::HashMap;

use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};

static TILE_EMPTY: char = '.';
static TILE_ELF: char = '#';
static DIRECTIONS: [Point; 12] = [
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
        let mut map = parse_map(ctx)?;

        // println!("\n== Initial State ==");
        // map.print();
        for round in 0..10 {
            map = execute_round(map, round);
            // println!("\n== After round {} ==", round + 1);
            // map.print();
        }

        let (min, max) = map.bounds();
        let empty_count = itertools::iproduct!(min.x..=max.x, min.y..=max.y)
            .filter(|(x, y)| map.tiles.get(&Point::new(*x, *y)).is_none())
            .count();

        Ok(empty_count.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

fn execute_round(prev: Map, round_idx: usize) -> Map {
    const TOTAL_DIRECTIONS: usize = 12;
    const CARDINAL_DIRECTIONS: usize = 4;
    const SUB_DIRECTIONS: usize = 3;

    let mut next = Map::default();
    let mut proposed: HashMap<Point, Vec<Point>> = HashMap::new();
    'elves: for (elf, _) in &prev.tiles {
        let start_dir = (round_idx * SUB_DIRECTIONS) % TOTAL_DIRECTIONS;

        let all_neighbors_empty = DIRECTIONS
            .iter()
            .all(|dir| prev.tiles.get(&(*elf + *dir)).is_none());
        if all_neighbors_empty {
            next.tiles.insert(*elf, TILE_ELF);
            continue;
        }

        'propose_dir: for cardinal_idx in 0..CARDINAL_DIRECTIONS {
            let cardinal_idx = (start_dir + cardinal_idx * SUB_DIRECTIONS) % TOTAL_DIRECTIONS;
            for sub_idx in 0..SUB_DIRECTIONS {
                let dir = DIRECTIONS[cardinal_idx + sub_idx];
                let neighbor = *elf + dir;
                if let Some(_) = prev.tiles.get(&neighbor) {
                    continue 'propose_dir;
                }
            }

            let dir = DIRECTIONS[cardinal_idx];
            let target = *elf + dir;
            proposed
                .entry(target)
                .and_modify(|x| x.push(*elf))
                .or_insert_with(|| Vec::from([*elf]));
            continue 'elves;
        }

        // cannot move
        next.tiles.insert(*elf, TILE_ELF);
    }

    for (target, elves) in proposed {
        match elves.len() {
            1 => _ = next.tiles.insert(target, TILE_ELF),
            _ => elves
                .into_iter()
                .for_each(|p| _ = next.tiles.insert(p, TILE_ELF)),
        };
    }

    next
}

fn parse_map(ctx: &Context) -> GenericResult<Map> {
    let mut tiles = HashMap::new();
    for (y, line) in ctx.input().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == TILE_ELF {
                tiles.insert(Point::new(x as i32, y as i32), c);
            }
        }
    }

    Ok(Map { tiles })
}

#[derive(Clone, Debug, Default)]
struct Map {
    tiles: HashMap<Point, char>,
}
impl Map {
    fn print(&self) {
        let (min, max) = self.bounds();
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                match self.tiles.get(&Point::new(x, y)) {
                    Some(c) => print!("{}", c),
                    None => print!("{}", TILE_EMPTY),
                }
            }
            println!();
        }
    }

    fn bounds(&self) -> (Point, Point) {
        let mut min = Point::new(i32::MAX, i32::MAX);
        let mut max = Point::new(i32::MIN, i32::MIN);
        for (p, _) in &self.tiles {
            min = min.min_parts(*p);
            max = max.max_parts(*p);
        }

        (min, max)
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
