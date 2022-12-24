use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{collections::HashMap, hash::Hash};

const TILE_WALL: char = '#';
const TILE_EMPTY: char = '.';
const TILE_BLIZZARDS: [char; 4] = ['>', 'v', '<', '^'];
const DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },  // East
    Point { x: 0, y: 1 },  // South
    Point { x: -1, y: 0 }, // West
    Point { x: 0, y: -1 }, // North
];
const DIRECTIONS_WITH_STAY: [Point; 5] = [
    Point { x: 1, y: 0 },  // East
    Point { x: 0, y: 1 },  // South
    Point { x: -1, y: 0 }, // West
    Point { x: 0, y: -1 }, // North
    Point { x: 0, y: 0 },  // Stay
];

#[derive(Default)]
pub struct Day24;
impl Solution for Day24 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 24, "Blizzard Basin")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_map(ctx)?;
        let time = traverse(&map, &map.start, &map.goal, 0, &mut HashMap::new())
            .ok_or("could not find path")?;
        Ok(time.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_map(ctx)?;

        let err = "could not find path";
        let mut blizzards_at = HashMap::new();
        ctx.progress(0.01);
        let time = traverse(&map, &map.start, &map.goal, 0, &mut blizzards_at).ok_or(err)?;
        ctx.progress(0.3333);
        let time = traverse(&map, &map.goal, &map.start, time, &mut blizzards_at).ok_or(err)?;
        ctx.progress(0.6666);
        let time = traverse(&map, &map.start, &map.goal, time, &mut blizzards_at).ok_or(err)?;

        Ok(time.to_string())
    }
}

fn traverse(
    map: &Map,
    start: &Point,
    goal: &Point,
    elapsed: i32,
    blizzards_at: &mut HashMap<i32, HashMap<Point, Vec<usize>>>,
) -> Option<i32> {
    blizzards_at.insert(0, map.starting_blizzards.clone());
    let mut queue = PriorityQueue::new();
    queue.push((elapsed, *start), 0);

    while let Some(((time, pos), _priority)) = queue.pop() {
        if pos == *goal {
            return Some(time);
        }

        let next_blizzards = match blizzards_at.get(&(time + 1)) {
            Some(blizzards) => blizzards,
            None => {
                let blizzards = move_blizzards(&map, blizzards_at.get(&time).unwrap());
                blizzards_at.insert(time + 1, blizzards);
                blizzards_at.get(&(time + 1)).unwrap()
            }
        };

        for dir in DIRECTIONS_WITH_STAY {
            let next_pos = pos + dir;
            if next_blizzards.get(&next_pos).is_none() {
                if let Some(&TILE_EMPTY) = map.tiles.get(&next_pos) {
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
    let input = ctx.input();
    let lines = input.lines().collect_vec();
    let mut tiles = HashMap::new();
    let mut blizzards = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
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

    let height = lines.len() - 2;
    let width = lines.get(0).ok_or("no input available")?.len() - 2;
    let start = (0..width)
        .map(|x| Point::new(x as i32, -1))
        .find(|p| *tiles.get(p).unwrap() == TILE_EMPTY)
        .ok_or("could not find start")?;
    let goal = (0..width)
        .map(|x| Point::new(x as i32, height as i32))
        .find(|p| *tiles.get(p).unwrap() == TILE_EMPTY)
        .ok_or("could not find start")?;

    Ok(Map {
        width,
        height,
        start,
        goal,
        tiles,
        starting_blizzards: blizzards,
    })
}

#[derive(Debug, Default, Clone)]
struct Map {
    width: usize,
    height: usize,
    start: Point,
    goal: Point,
    tiles: HashMap<Point, char>,
    starting_blizzards: HashMap<Point, Vec<usize>>,
}
impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        println!();
        for y in -1..self.height as i32 + 1 {
            for x in -1..self.width as i32 + 1 {
                let p = Point::new(x, y);
                match self.starting_blizzards.get(&p) {
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
