use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};

static DIRECTIONS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

#[derive(Default)]
pub struct Day08 {
    map: Vec<Vec<i32>>,
    width: usize,
    height: usize,
}
impl Solution for Day08 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 8, "Treetop Tree House")
    }

    fn init(&mut self, ctx: &Context) -> GenericResult {
        self.map = ctx
            .input()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).and_then(|x| Some(x as i32)))
                    .collect()
            })
            .collect::<Option<_>>()
            .ok_or("invalid input")?;
        self.width = self.map.get(0).and_then(|x| Some(x.len())).unwrap_or(0);
        self.height = self.map.len();

        Ok(())
    }

    fn part1(&mut self, _ctx: &Context) -> SolutionResult {
        let visible_count = itertools::iproduct!(0..self.width, 0..self.height)
            .map(|(x, y)| {
                let p = Point::new(x as i32, y as i32);
                let value = self.map[y][x];

                match DIRECTIONS
                    .iter()
                    .find(|d| self.get_tallest(&p, d).0 < value)
                {
                    Some(_) => 1,
                    None => 0,
                }
            })
            .sum::<i32>();

        Ok(visible_count.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        let max_score = itertools::iproduct!(0..self.width, 0..self.height)
            .map(|(x, y)| {
                let p = Point::new(x as i32, y as i32);
                let score = DIRECTIONS
                    .iter()
                    .fold(1, |a, d| a * self.get_tallest(&p, d).1);
                score
            })
            .max()
            .ok_or("map is empty")?;
        Ok(max_score.to_string())
    }
}

impl Day08 {
    fn get_tallest(&self, start: &Point, dir: &Point) -> (i32, i32) {
        let start_value = self.map[start.y as usize][start.x as usize];
        let mut pos = start.clone();
        let mut max = -1;
        let mut count = 0;
        loop {
            pos += *dir;
            if pos.x < 0 || pos.x >= self.width as i32 || pos.y < 0 || pos.y >= self.height as i32 {
                break;
            }

            count += 1;
            let current = self.map[pos.y as usize][pos.x as usize];
            max = max.max(current);
            if max >= start_value {
                break;
            }
        }

        return (max, count);
    }
}

#[derive(Copy, Clone, Debug, Constructor, Add, Sub, AddAssign, SubAssign)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
