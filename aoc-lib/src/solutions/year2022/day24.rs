use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use priority_queue::PriorityQueue;
use std::collections::HashMap;

const TILE_WALL: char = '#';
const TILE_EMPTY: char = '.';
const TILE_BLIZZARDS: [char; 4] = ['>', 'v', '<', '^'];
const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },  // East
    Point { x: 0, y: 1 },  // South
    Point { x: -1, y: 0 }, // West
    Point { x: 0, y: -1 }, // North
];
const DIRECTIONS_AND_STAY: [Point; 5] = [
    Point { x: 1, y: 0 },  // East
    Point { x: 0, y: 1 },  // South
    Point { x: -1, y: 0 }, // West
    Point { x: 0, y: -1 }, // North
    Point { x: 0, y: 0 },  // No
];

#[derive(Default)]
pub struct Day24;
impl Solution for Day24 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 24, "Blizzard Basin")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_map(ctx)?;
        let time = bfs(&map).ok_or("could not find path")?;
        Ok(time.to_string()) // 370 too high
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

fn bfs(map: &Map) -> Option<i32> {
    let mut blizzards_at = HashMap::from([(0, map.blizzards.clone())]);
    let mut queue = PriorityQueue::new();
    queue.push((1, map.start), -map.start.manhattan(&map.goal));

    while let Some(((time, pos), _priority)) = queue.pop() {
        let next_blizzards = match blizzards_at.get(&time) {
            Some(blizzards) => blizzards,
            None => {
                let blizzards = move_blizzards(&map, blizzards_at.get(&(time - 1)).unwrap());
                blizzards_at.insert(time, blizzards);
                blizzards_at.get(&time).unwrap()
            }
        };

        for dir in DIRECTIONS_AND_STAY {
            let next_pos = pos + dir;
            if next_blizzards.get(&next_pos).is_none() {
                if let Some(&TILE_EMPTY) = map.tiles.get(&next_pos) {
                    if next_pos == map.goal {
                        return Some(time);
                    }

                    // queue.push_back((time + 1, next_pos));
                    queue.push((time + 1, next_pos), -next_pos.manhattan(&map.goal) - time);
                }
            }
        }
    }

    None
}

fn move_blizzards(map: &Map, blizzards: &HashMap<Point, Vec<usize>>) -> HashMap<Point, Vec<usize>> {
    let mut next = HashMap::new();
    for (pos, blizzards_at_pos) in blizzards {
        for blizzard in blizzards_at_pos {
            let dir = DIRECTIONS[*blizzard];
            let next_pos = Point::new(
                (pos.x + dir.x + map.width as i32) % map.width as i32,
                (pos.y + dir.y + map.height as i32) % map.height as i32,
            );
            next.entry(next_pos)
                .or_insert_with(|| Vec::new())
                .push(*blizzard);
        }
    }

    next
}

fn parse_map(ctx: &Context) -> GenericResult<Map> {
    let mut tiles = HashMap::new();
    let mut blizzards = HashMap::new();
    for (y, line) in ctx.input().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let p = Point::new(x as i32 - 1, y as i32 - 1);
            match c {
                _ if c == TILE_WALL => _ = tiles.insert(p, c),
                _ if c == TILE_EMPTY => _ = tiles.insert(p, c),
                _ => {
                    tiles.insert(p, TILE_EMPTY);
                    blizzards.insert(
                        p,
                        vec![TILE_BLIZZARDS
                            .iter()
                            .position(|x| *x == c)
                            .ok_or("invalid input")?],
                    );
                }
            }
        }
    }

    let width = ctx
        .input()
        .lines()
        .next()
        .and_then(|x| Some(x.len()))
        .unwrap_or(2)
        - 2;
    let height = ctx.input().lines().count() - 2;
    let start = (0..width)
        .map(|x| Point::new(x as i32, -1))
        .find(|p| {
            tiles
                .get(p)
                .and_then(|c| Some(*c == TILE_EMPTY))
                .unwrap_or(false)
        })
        .ok_or("could not find start")?;
    let goal = (0..width)
        .map(|x| Point::new(x as i32, height as i32))
        .find(|p| {
            tiles
                .get(p)
                .and_then(|c| Some(*c == TILE_EMPTY))
                .unwrap_or(false)
        })
        .ok_or("could not find goal")?;

    Ok(Map {
        width,
        height,
        start,
        goal,
        tiles,
        blizzards,
    })
}

#[derive(Debug, Default, Clone)]
struct Map {
    width: usize,
    height: usize,
    start: Point,
    goal: Point,
    tiles: HashMap<Point, char>,
    blizzards: HashMap<Point, Vec<usize>>,
}
impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        println!();
        for y in -1..self.height as i32 + 1 {
            for x in -1..self.width as i32 + 1 {
                let p = Point::new(x, y);
                match self.blizzards.get(&p) {
                    Some(blizzards) => match blizzards.len() {
                        1 => print!("{}", TILE_BLIZZARDS[blizzards[0]]),
                        _ => print!("{}", blizzards.len().min(9)),
                    },
                    None => match self.tiles.get(&p) {
                        Some(c) => print!("{}", c),
                        None => print!(" "),
                    },
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
    pub fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
