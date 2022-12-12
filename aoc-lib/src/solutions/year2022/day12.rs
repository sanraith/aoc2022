use crate::solution::*;
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const CHARCODE_LOWERCASE_A: i32 = 97;
const CHARCODE_LOWERCASE_Z: i32 = 122;

#[derive(Default)]
pub struct Day12;
impl Solution for Day12 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 12, "Hill Climbing Algorithm")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_height_map(ctx);
        let fewest_steps = find_shortest_path(&map, map.start).ok_or("no path exists")?;

        Ok(fewest_steps.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_height_map(ctx);
        let shortest_path = itertools::iproduct!(0..map.width, 0..map.height)
            .filter(|(x, y)| map.tiles[*y][*x] == 0)
            .filter_map(|(x, y)| {
                ctx.progress((x * map.height + y) as f32 / (map.width * map.height) as f32);
                find_shortest_path(&map, Point::new(x as i32, y as i32))
            })
            .sorted()
            .next()
            .ok_or("no path exists")?;

        Ok(shortest_path.to_string())
    }
}

fn find_shortest_path(map: &Map, start: Point) -> Option<i32> {
    let mut visited: HashSet<Point> = HashSet::from_iter([start]);
    let mut stack = VecDeque::from_iter([(start, 0)]);
    let directions = [
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(-1, 0),
    ];

    while let Some((pos, steps)) = stack.pop_front() {
        if pos == map.end {
            return Some(steps);
        }

        let height = map.tiles[pos.y as usize][pos.x as usize];
        for direction in &directions {
            let next_pos = pos + *direction;
            if visited.contains(&next_pos)
                || next_pos.x < 0
                || next_pos.x >= map.width as i32
                || next_pos.y < 0
                || next_pos.y >= map.height as i32
            {
                continue;
            }

            let next_height = map.tiles[next_pos.y as usize][next_pos.x as usize];
            if next_height - height <= 1 {
                stack.push_back((next_pos, steps + 1));
                visited.insert(next_pos);
            }
        }
    }

    return None;
}

fn parse_height_map(ctx: &Context) -> Map {
    let input = ctx.input();
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines.get(0).map(|l| l.len()).unwrap_or(0);

    let mut start = Point::default();
    let mut end = Point::default();
    let mut tiles = Vec::with_capacity(height);
    for (y, line) in lines.into_iter().enumerate() {
        tiles.push(Vec::with_capacity(width));
        for (x, c) in line.chars().enumerate() {
            let value = match c {
                'S' => {
                    start = Point::new(x as i32, y as i32);
                    0
                }
                'E' => {
                    end = Point::new(x as i32, y as i32);
                    CHARCODE_LOWERCASE_Z - CHARCODE_LOWERCASE_A
                }
                _ => c as i32 - 97,
            };
            tiles[y].push(value);
        }
    }

    Map {
        width,
        height,
        start,
        end,
        tiles,
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    start: Point,
    end: Point,
    tiles: Vec<Vec<i32>>,
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
