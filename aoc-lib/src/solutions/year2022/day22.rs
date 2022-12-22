use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Mul, MulAssign, Sub, SubAssign};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, ops::RangeInclusive};

static TILE_VOID: char = ' ';
static TILE_OPEN: char = '.';
static TILE_WALL: char = '#';
static FACINGS: [char; 4] = ['R', 'D', 'L', 'U'];
static DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

// static DIRECTION_POINTS: Lazy<HashMap<char, Point>> = Lazy::new(|| {
//     HashMap::from([
//         ('U', Point::new(0, -1)),
//         ('D', Point::new(0, 1)),
//         ('L', Point::new(-1, 0)),
//         ('R', Point::new(1, 0)),
//     ])
// });

#[derive(Default)]
pub struct Day22;
impl Solution for Day22 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 22, "Monkey Map")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_input(ctx)?;
        let (pos, facing) = walk(&map)?;
        let password = (pos.y + 1) * 1000 + (pos.x + 1) * 4 + facing as i32;
        Ok(password.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

fn walk(map: &Map) -> GenericResult<(Point, usize)> {
    let mut pos = map.start.clone();
    let mut facing = 0;
    for (facing_next, distance) in &map.path {
        facing = *facing_next;
        for _ in 0..*distance {
            let next_pos = get_next_pos(pos, facing, map)?;
            match map.tiles.get(&next_pos) {
                Some(&c) if c == TILE_WALL => break,
                Some(_) => pos = next_pos,
                None => Err("should not arrive to void")?,
            }
        }
    }

    Ok((pos, facing))
}

fn get_next_pos(start: Point, facing: usize, map: &Map) -> GenericResult<Point> {
    let err = "could not find wrap-around position";
    let pos = start + DIRECTIONS[facing];
    let next_pos = match map.tiles.get(&pos) {
        Some(_) => pos,
        None => match facing {
            0 => Point::new(*map.ranges_x.get(&pos.y).ok_or(err)?.start(), pos.y),
            1 => Point::new(pos.x, *map.ranges_y.get(&pos.x).ok_or(err)?.start()),
            2 => Point::new(*map.ranges_x.get(&pos.y).ok_or(err)?.end(), pos.y),
            3 => Point::new(pos.x, *map.ranges_y.get(&pos.x).ok_or(err)?.end()),
            _ => Err("invalid facing")?,
        },
    };

    Ok(next_pos)
}

fn parse_input(ctx: &Context) -> GenericResult<Map> {
    let input = ctx.input();
    let (input_map, input_path) = input.split_once("\n\n").ok_or("invalid input")?;

    let mut tiles = HashMap::new();
    let mut ranges_x: HashMap<i32, RangeInclusive<i32>> = HashMap::new();
    let mut ranges_y: HashMap<i32, RangeInclusive<i32>> = HashMap::new();

    for (y, line) in input_map.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if c == TILE_OPEN || c == TILE_WALL {
                tiles.insert(Point::new(x as i32, y as i32), c);
                ranges_x
                    .entry(y)
                    .and_modify(|r| *r = *r.start().min(&x)..=*r.end().max(&x))
                    .or_insert(x..=x);
                ranges_y
                    .entry(x)
                    .and_modify(|r| *r = *r.start().min(&y)..=*r.end().max(&y))
                    .or_insert(y..=y);
            }
        }
    }

    let start_x = ranges_x
        .get(&0)
        .ok_or("invalid input")?
        .clone()
        .find(|x| {
            tiles
                .get(&Point::new(*x as i32, 0))
                .and_then(|&c| Some(c == TILE_OPEN))
                .unwrap_or(false)
        })
        .ok_or("could not find start tile")? as i32;

    let path_part_re = Regex::new(r"(-?\d+)([LR])?")?;
    let mut path = Vec::new();
    let mut facing = 0;
    for captures in path_part_re.captures_iter(input_path) {
        let distance = captures
            .get(1)
            .and_then(|x| x.as_str().parse::<i32>().ok())
            .ok_or("invalid path")?;
        path.push((facing, distance));

        let turn = captures.get(2).and_then(|x| x.as_str().chars().next());
        match turn {
            Some(c) if c == 'R' => facing = (facing + 1) % 4,
            Some(c) if c == 'L' => facing = (facing + 3) % 4,
            Some(_) => Err("invalid path")?,
            None => (),
        }
    }

    Ok(Map {
        start: Point::new(start_x, 0),
        tiles,
        ranges_x,
        ranges_y,
        path,
    })
}

#[derive(Debug)]
struct Map {
    start: Point,
    tiles: HashMap<Point, char>,
    path: Vec<(usize, i32)>,
    ranges_x: HashMap<i32, RangeInclusive<i32>>,
    ranges_y: HashMap<i32, RangeInclusive<i32>>,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    Constructor,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    Mul,
    MulAssign,
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
