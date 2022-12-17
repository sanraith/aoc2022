use crate::{
    solution::*,
    util::{DynError, GenericResult},
};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashMap, str::FromStr};

static TILE_AIR: char = '.';
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
        let mut cave = parse_cave(ctx)?;
        rock_fall(&mut cave, 2022);
        let height = (cave.top_left.y - cave.bottom_right.y).abs();

        Ok(height.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

fn rock_fall(cave: &mut Cave, target_fallen_count: i32) {
    let mut rocks = ROCK_SHAPES.clone();
    let rock_type_count = rocks.len();
    let mut rock_index = 0;
    let mut wind_index = 0;

    rocks[rock_index].pos = Point::new(
        cave.top_left.x + 2,
        cave.top_left.y - 3 - rocks[rock_index].height,
    );
    let mut fallen_count = 0;
    while fallen_count < target_fallen_count {
        let rock = &mut rocks[rock_index];

        rock.pos.x += cave.wind[wind_index];
        if cave.intersecting(rock) {
            rock.pos.x -= cave.wind[wind_index];
        }

        rock.pos.y += 1;
        if cave.intersecting(rock) {
            rock.pos.y -= 1;
            cave.land(&rock);
            fallen_count += 1;
            // print_cave(cave);
            // println!("{:?},{:?}", cave.top_left, cave.bottom_right);

            rock_index = (rock_index + 1) % rock_type_count;
            rocks[rock_index].pos = Point::new(
                cave.top_left.x + 2,
                cave.top_left.y - 3 - rocks[rock_index].height,
            );
        }

        wind_index = (wind_index + 1) % cave.wind.len();
    }
}

fn print_cave(cave: &Cave) {
    for y in cave.top_left.y..=cave.bottom_right.y {
        for x in cave.top_left.x..=cave.bottom_right.x {
            match cave.tiles.get(&Point::new(x, y)) {
                Some(c) => print!("{}", c),
                None => print!("{}", TILE_AIR),
            }
        }
        println!();
    }
    println!();
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

    // let tiles: HashMap<Point, char> =
    //     HashMap::from_iter((0..7).map(|x| (Point::new(x, 0), TILE_ROCK)));
    let tiles = HashMap::new();

    Ok(Cave {
        top_left: Point::new(0, 0),
        bottom_right: Point::new(6, 0),
        tiles,
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
impl Cave {
    fn intersecting(&self, rock: &RockShape) -> bool {
        let out_of_bounds = rock.pos.x < self.top_left.x
            || rock.pos.x + rock.width - 1 > self.bottom_right.x
            || rock.pos.y + rock.height > self.bottom_right.y;

        out_of_bounds
            || rock
                .tiles
                .iter()
                .any(|p| self.tiles.contains_key(&(*p + rock.pos)))
    }

    fn land(&mut self, rock: &RockShape) {
        for p in &rock.tiles {
            let p = *p + rock.pos;
            self.tiles.insert(p, TILE_ROCK);
            self.top_left.x = self.top_left.x.min(p.x);
            self.top_left.y = self.top_left.y.min(p.y);
            self.bottom_right.x = self.bottom_right.x.max(p.x);
            self.bottom_right.y = self.bottom_right.y.max(p.y + 1);
        }
    }
}

#[derive(Debug, Clone)]
struct RockShape {
    pos: Point,
    width: i32,
    height: i32,
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
            pos: Point::default(),
            width: lines.get(0).unwrap_or(&"").len() as i32,
            height: lines.len() as i32,
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
