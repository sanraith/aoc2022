use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Day12;
impl Solution for Day12 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 12, "Hill Climbing Algorithm")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_height_map(ctx)?;
        let (step_map, _) = find_paths(&map);
        let fewest_steps = step_map[map.start.y as usize][map.start.x as usize]
            .ok_or("no path available from start")?;

        Ok(fewest_steps.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_height_map(ctx)?;
        let (_, shortest_path) = find_paths(&map);
        let shortest_path = shortest_path.ok_or("no path available from any elevation 'a'")?;

        Ok(shortest_path.to_string())
    }
}

fn find_paths(map: &Map) -> (Vec<Vec<Option<i32>>>, Option<i32>) {
    let directions = [
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(-1, 0),
        Point::new(0, -1),
    ];
    let mut min_steps = None;
    let mut stack = VecDeque::from([(map.end, 0)]);
    let mut step_map = vec![vec![None::<i32>; map.width]; map.height];

    while let Some((pos, steps)) = stack.pop_front() {
        let height = map.tiles[pos.y as usize][pos.x as usize];
        if height == 0 && steps < min_steps.unwrap_or(i32::MAX) {
            min_steps = Some(steps);
        }

        for direction in &directions {
            let next_pos = pos + *direction;
            if next_pos.x < 0
                || next_pos.x >= map.width as i32
                || next_pos.y < 0
                || next_pos.y >= map.height as i32
                || step_map[next_pos.y as usize][next_pos.x as usize].is_some()
            {
                continue;
            }

            let next_height = map.tiles[next_pos.y as usize][next_pos.x as usize];
            if height - next_height <= 1 {
                stack.push_back((next_pos, steps + 1));
                step_map[next_pos.y as usize][next_pos.x as usize] = Some(steps + 1);
            }
        }
    }

    (step_map, min_steps)
}

fn parse_height_map(ctx: &Context) -> GenericResult<Map> {
    let input = ctx.input();
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines.get(0).map(|line| line.len()).unwrap_or(0);

    let mut start = None;
    let mut end = None;
    let mut tiles = vec![vec![]; height];
    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            tiles[y].push(match c {
                'a'..='z' => c as i32 - b'a' as i32,
                'S' => {
                    start = Some(Point::new(x as i32, y as i32));
                    0
                }
                'E' => {
                    end = Some(Point::new(x as i32, y as i32));
                    (b'z' - b'a') as i32
                }
                _ => Err("invalid character in input")?,
            });
        }
    }

    Ok(Map {
        width,
        height,
        start: start.ok_or("start not found in input")?,
        end: end.ok_or("end not found in input")?,
        tiles,
    })
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
