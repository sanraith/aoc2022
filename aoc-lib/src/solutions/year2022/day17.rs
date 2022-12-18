use crate::{
    solution::*,
    util::{DynError, GenericResult},
};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

static CAVE_WIDTH: i64 = 7;
static TILE_AIR: char = '.';
static TILE_ROCK: char = '#';
static ROCK_KINDS: Lazy<Vec<RockShape>> = Lazy::new(|| {
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
        drop_rocks(ctx, &mut cave, 2022);
        Ok(cave.height().to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let mut cave = parse_cave(ctx)?;
        drop_rocks(ctx, &mut cave, 1000000000000);
        Ok(cave.height().to_string())
    }
}

fn drop_rocks(_ctx: &Context, cave: &mut Cave, target_drop_count: i64) {
    let top_count =
        ROCK_KINDS.len() as i64 * ROCK_KINDS.iter().map(|x| x.height).max().unwrap_or(0);

    // (wind_idx, cave_top) -> (count, height)
    let mut checkpoints: HashMap<(usize, String), (i64, i64)> = HashMap::new();
    let mut drop_count = 0;
    let mut rock_idx = 0;
    let mut wind_idx = 0;
    let mut rock = drop_rock(cave, rock_idx);

    while drop_count < target_drop_count {
        // handle wind
        rock.pos.x += cave.wind[wind_idx];
        if cave.intersecting(&rock) {
            rock.pos.x -= cave.wind[wind_idx];
        }
        wind_idx = (wind_idx + 1) % cave.wind.len();

        // handle fall
        rock.pos.y += 1;
        if !cave.intersecting(&rock) {
            continue;
        }
        rock.pos.y -= 1;
        drop_count += 1;
        cave.land(&rock);

        // Jump over repeating patterns
        if (drop_count % ROCK_KINDS.len() as i64) == 0 {
            if let Some((prev_drop_count, prev_height)) = checkpoints.insert(
                (wind_idx, cave_top(cave, top_count)),
                (drop_count, cave.height()),
            ) {
                let cycle_size = drop_count - prev_drop_count;
                let cycles = (target_drop_count - drop_count) / cycle_size;
                drop_count += cycles * cycle_size;

                let copy_from = cave.top_left.y..=cave.top_left.y + top_count;
                let copy_to = cave.top_left.y - (cave.height() - prev_height) * cycles;
                copy_cave_part(cave, copy_from, copy_to);

                checkpoints.clear();
            }
        }

        // move on to next rock
        rock_idx = (rock_idx + 1) % ROCK_KINDS.len();
        rock = drop_rock(cave, rock_idx);
    }
}

/// Create a new rock of the given kind at the correct height.
fn drop_rock(cave: &Cave, rock_idx: usize) -> RockShape {
    let mut rock = ROCK_KINDS[rock_idx].clone();
    rock.pos = Point::new(cave.top_left.x + 2, cave.top_left.y - rock.height - 3);
    rock
}

/// Copy a vertical slice of the cave to another y location within the cave.
fn copy_cave_part(cave: &mut Cave, y_from_range: RangeInclusive<i64>, y_to: i64) {
    for (y_index, y_from) in y_from_range.enumerate() {
        for x in cave.top_left.x..=cave.bottom_right.x {
            if let Some(tile) = cave.tiles.get(&Point::new(x, y_from)) {
                cave.insert_tile(Point::new(x, y_to + y_index as i64), *tile);
            }
        }
    }
}

/// Get the top n rows of the cave as a single string.
fn cave_top(cave: &Cave, top_row_count: i64) -> String {
    itertools::iproduct!(
        cave.top_left.y..=cave.top_left.y + top_row_count,
        cave.top_left.x..=cave.bottom_right.x
    )
    .map(|(y, x)| match cave.tiles.get(&Point::new(x, y)) {
        Some(c) => *c,
        None => TILE_AIR,
    })
    .collect::<String>()
}

/// Print the cave to the standard output.
#[allow(dead_code)]
fn print_cave(cave: &Cave) {
    let cave_str = cave_top(cave, (cave.top_left.y - cave.bottom_right.y).abs())
        .chars()
        .chunks(CAVE_WIDTH as usize)
        .into_iter()
        .map(|x| x.collect::<String>())
        .join("\n");
    println!("{}", cave_str);
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
        bottom_right: Point::new(CAVE_WIDTH - 1, 0),
        tiles: HashMap::from_iter((0..CAVE_WIDTH).map(|x| (Point::new(x, 0), TILE_ROCK))),
        wind,
    })
}

#[derive(Debug, Default)]
struct Cave {
    top_left: Point,
    bottom_right: Point,
    tiles: HashMap<Point, char>,
    wind: Vec<i64>,
}
impl Cave {
    /// True, if the given rock is intersecting with other rocks or the walls of the cave.
    fn intersecting(&self, rock: &RockShape) -> bool {
        rock.pos.x < self.top_left.x
            || rock.pos.x + rock.width - 1 > self.bottom_right.x
            || rock
                .tiles
                .iter()
                .any(|p| self.tiles.contains_key(&(*p + rock.pos)))
    }

    /// Place a copy of the rock at its current position withing the cave.
    fn land(&mut self, rock: &RockShape) {
        for p in &rock.tiles {
            self.insert_tile(*p + rock.pos, TILE_ROCK);
        }
    }

    /// Update the bounds of the cave
    fn insert_tile(&mut self, p: Point, tile: char) {
        self.tiles.insert(p, tile);
        self.top_left.x = self.top_left.x.min(p.x);
        self.top_left.y = self.top_left.y.min(p.y);
        self.bottom_right.x = self.bottom_right.x.max(p.x);
        self.bottom_right.y = self.bottom_right.y.max(p.y + 1);
    }

    /// The height of the cave
    fn height(&self) -> i64 {
        (self.top_left.y - self.bottom_right.y).abs()
    }
}

#[derive(Debug, Clone)]
struct RockShape {
    pos: Point,
    width: i64,
    height: i64,
    tiles: Vec<Point>,
}
impl FromStr for RockShape {
    type Err = DynError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        let mut tiles = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, _) in line.chars().enumerate().filter(|(_, c)| *c == TILE_ROCK) {
                tiles.push(Point::new(x as i64, y as i64));
            }
        }
        tiles.reverse(); // Earlier intersection hits this way

        Ok(RockShape {
            pos: Point::default(),
            width: lines.get(0).unwrap_or(&"").len() as i64,
            height: lines.len() as i64,
            tiles,
        })
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point {
    pub x: i64,
    pub y: i64,
}
