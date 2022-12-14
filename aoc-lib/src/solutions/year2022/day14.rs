use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[allow(dead_code)]
static TILE_AIR: char = ' ';
static TILE_ROCK: char = '#';
static TILE_SAND: char = 'o';
static SAND_START: Point = Point { x: 500, y: 0 };
static SAND_DIRECTIONS: Lazy<[Point; 3]> =
    Lazy::new(|| [Point::new(0, 1), Point::new(-1, 1), Point::new(1, 1)]);

#[derive(Default)]
pub struct Day14;
impl Solution for Day14 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 14, "Regolith Reservoir")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let mut cave = parse_cave(ctx)?;
        let mut count = 0;
        while fall_sand(&SAND_START, &mut cave, false) {
            count += 1;
        }

        Ok(count.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let mut cave = parse_cave(ctx)?;
        let mut count = 0;
        while fall_sand(&SAND_START, &mut cave, true) {
            count += 1;
        }

        Ok(count.to_string())
    }
}

fn fall_sand(pos: &Point, cave: &mut Cave, has_floor: bool) -> bool {
    if let Some(_) = cave.map.get(&pos) {
        return false;
    }

    let mut pos = *pos;
    let mut rested = true;
    while let Some(next) = SAND_DIRECTIONS
        .iter()
        .map(|dir| pos + *dir)
        .find(|p| !cave.map.contains_key(&p))
    {
        pos = next;
        if !has_floor && next.y > cave.bottom_right.y {
            rested = false;
            break;
        }
        if has_floor && next.y == cave.floor - 1 {
            break;
        }
    }

    if rested {
        cave.map.insert(pos, TILE_SAND);
        update_bounds(cave, &pos);
    }

    rested
}

fn parse_cave(ctx: &Context) -> GenericResult<Cave> {
    let mut cave = Cave {
        top_left: Point::new(i32::MAX, i32::MAX),
        bottom_right: Point::new(i32::MIN, i32::MIN),
        ..Default::default()
    };

    for line in ctx.input().lines() {
        let segments = line
            .split(" -> ")
            .filter_map(|p| {
                p.split(",")
                    .filter_map(|n| n.parse::<i32>().ok())
                    .collect_tuple()
            })
            .map(|(x, y)| Point::new(x, y))
            .tuple_windows();
        for (a, b) in segments {
            let coords =
                itertools::iproduct!(a.x.min(b.x)..=a.x.max(b.x), a.y.min(b.y)..=a.y.max(b.y));
            for (x, y) in coords {
                cave.map.insert(Point::new(x, y), TILE_ROCK);
                update_bounds(&mut cave, &Point::new(x, y));
            }
        }
    }

    cave.floor = cave.bottom_right.y + 2;
    Ok(cave)
}

fn update_bounds(cave: &mut Cave, last_point: &Point) {
    cave.top_left.x = cave.top_left.x.min(last_point.x);
    cave.top_left.y = cave.top_left.y.min(last_point.y);
    cave.bottom_right.x = cave.bottom_right.x.max(last_point.x);
    cave.bottom_right.y = cave.bottom_right.y.max(last_point.y);
}

fn _print_cave(cave: &Cave) {
    println!("{:?}..{:?}", cave.top_left, cave.bottom_right);
    for y in cave.top_left.y..=cave.bottom_right.y {
        for x in cave.top_left.x..=cave.bottom_right.x {
            match cave.map.get(&Point::new(x, y)) {
                Some(c) => print!("{}", c),
                None => print!("{}", TILE_AIR),
            }
        }
        println!("");
    }
}

#[derive(Debug, Default)]
struct Cave {
    map: HashMap<Point, char>,
    top_left: Point,
    bottom_right: Point,
    floor: i32,
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
