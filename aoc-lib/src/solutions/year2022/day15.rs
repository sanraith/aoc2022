use crate::{helpers::re_capture_groups, solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use regex::Regex;
use std::ops::Range;

#[derive(Default)]
pub struct Day15 {
    pub part1_scan_y: i64,
    pub part2_scan_range: i64,
}
impl Solution for Day15 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 15, "Beacon Exclusion Zone")
    }

    fn init(&mut self, _ctx: &Context) -> GenericResult {
        self.part1_scan_y = 2000000;
        self.part2_scan_range = 4000000;
        Ok(())
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let sensors = parse_input(ctx)?;
        let coverages = sensors
            .iter()
            .filter_map(|x| x.coverage_at_y(self.part1_scan_y))
            .collect_vec();

        let (covered_area, ..) = covered_area(coverages)?;
        let beacons_in_area = sensors
            .iter()
            .filter(|s| s.closest_beacon.y == self.part1_scan_y)
            .map(|x| x.closest_beacon)
            .dedup()
            .count() as i64;
        let position_count = covered_area - beacons_in_area;

        Ok(position_count.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let scan_range = 0..self.part2_scan_range + 1;
        let sensors = parse_input(ctx)?;

        let beacon_pos = scan_range
            .clone()
            .rev()
            .map(|y| {
                ctx.progress((scan_range.end - y) as f32 / scan_range.end as f32);

                let coverages = sensors
                    .iter()
                    .filter_map(|x| x.coverage_at_y(y))
                    .collect_vec();
                let (_, parts) = covered_area(coverages).ok()?;

                parts
                    .iter()
                    .filter(|p| p.start <= scan_range.end && p.end >= scan_range.start)
                    .sorted_by(|a, b| a.start.cmp(&b.start))
                    .tuple_windows()
                    .find(|(a, b)| a.end != b.start)
                    .map(|(a, _)| Point::new(a.end, y))
            })
            .find_map(|p| p)
            .ok_or("could not find beacon")?;
        let tuning_frequency = beacon_pos.x * 4000000 + beacon_pos.y;

        Ok(tuning_frequency.to_string())
    }
}

fn covered_area(mut ranges: Vec<Range<i64>>) -> GenericResult<(i64, Vec<Range<i64>>)> {
    let mut covered_parts = Vec::<Range<i64>>::new();
    'next_range: while let Some(mut range) = ranges.pop() {
        for covered in &covered_parts {
            let overlap = range.start.max(covered.start)..range.end.min(covered.end);
            if overlap.start >= overlap.end {
                continue;
            }

            match overlap {
                _ if covered.start <= range.start && covered.end >= range.end => {
                    // full range already covered
                    continue 'next_range;
                }
                _ if range.start <= covered.start && range.end >= covered.end => {
                    // cut away middle
                    ranges.push(range.start..covered.start);
                    ranges.push(covered.end..range.end);
                    continue 'next_range;
                }
                _ if covered.start <= range.start => range = covered.end..range.end, // cut away start
                _ if covered.end >= range.end => range = range.start..covered.start, // cut away end
                _ => Err("unhandled range overlap")?,
            };
        }

        if range.start < range.end {
            covered_parts.push(range);
        }
    }

    let area = covered_parts.iter().fold(0, |a, x| a + x.end - x.start);
    Ok((area, covered_parts))
}

fn parse_input(ctx: &Context) -> GenericResult<Vec<Sensor>> {
    let sensor_re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let mut sensors = Vec::new();
    for line in ctx.input().lines() {
        let (sx, sy, bx, by) = re_capture_groups(&sensor_re, line)
            .ok_or("invalid input")?
            .into_iter()
            .filter_map(|x| x.parse::<i64>().ok())
            .collect_tuple()
            .ok_or("invalid input")?;

        let pos = Point::new(sx, sy);
        let closest_beacon = Point::new(bx, by);
        sensors.push(Sensor {
            pos,
            closest_beacon,
            coverage: pos.manhattan(&closest_beacon),
        });
    }

    Ok(sensors)
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    closest_beacon: Point,
    coverage: i64,
}
impl Sensor {
    pub fn coverage_at_y(&self, y: i64) -> Option<Range<i64>> {
        let relative_y = (y - self.pos.y).abs();
        let half_width = self.coverage - relative_y;
        match half_width {
            _ if half_width < 0 => None,
            _ => Some(self.pos.x - half_width..self.pos.x + half_width + 1),
        }
    }
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point {
    pub x: i64,
    pub y: i64,
}
impl Point {
    pub fn manhattan(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
