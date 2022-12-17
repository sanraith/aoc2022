use crate::{
    solution::*,
    util::{DynError, GenericResult},
};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashMap, str::FromStr};

static TILE_AIR: char = ' ';
static TILE_ROCK: char = '#';
static ROCK_SHAPES: Lazy<Vec<RockShape>> = Lazy::new(|| {
    [
        "####",
        ".#.\n###\n.#.",
        "..#\n..#\n###",
        "#\n#\n#\n#",
        "##\n##",
    ]
    .iter()
    .filter_map(|x| RockShape::from_str(x).ok())
    .collect_vec()
});

#[derive(Default)]
pub struct Day17;
impl Solution for Day17 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 17, "Pyroclastic Flow")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let cave = parse_cave(ctx)?;
        println!("{:?}", cave);

        Ok("".to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

fn parse_cave(ctx: &Context) -> GenericResult<Cave> {
    let wind = ctx
        .input()
        .chars()
        .map(|c| match c {
            '<' => Some(-1),
            '>' => Some(1),
            _ => None,
        })
        .collect::<Option<_>>()
        .ok_or("invalid input")?;

    Ok(Cave {
        top_left: Point::new(0, 0),
        bottom_right: Point::new(6, 0),
        tiles: HashMap::new(),
        wind,
    })
}

#[derive(Debug, Default)]
struct Cave {
    top_left: Point,
    bottom_right: Point,
    tiles: HashMap<Point, char>,
    wind: Vec<i32>,
}

#[derive(Debug)]
struct RockShape {
    width: usize,
    height: usize,
    tiles: Vec<Point>,
}
impl FromStr for RockShape {
    type Err = DynError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let mut tiles = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == TILE_ROCK) {
                tiles.push(Point::new(x as i32, y as i32));
            }
        }

        // Reverse tiles for more efficient comparison. Points iteration: bottom_right -> top_left
        tiles.reverse();

        Ok(RockShape {
            width: lines.get(0).unwrap_or(&"").len(),
            height: lines.len(),
            tiles,
        })
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point {
    pub x: i32,
    pub y: i32,
}
