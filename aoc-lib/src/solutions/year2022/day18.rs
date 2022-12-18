use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Sub, SubAssign};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

static DIRECTIONS: [Point3; 6] = [
    Point3 { x: 1, y: 0, z: 0 },
    Point3 { x: -1, y: 0, z: 0 },
    Point3 { x: 0, y: 1, z: 0 },
    Point3 { x: 0, y: -1, z: 0 },
    Point3 { x: 0, y: 0, z: 1 },
    Point3 { x: 0, y: 0, z: -1 },
];

#[derive(Default)]
pub struct Day18;
impl Solution for Day18 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 18, "Boiling Boulders")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let voxels = parse_voxels(ctx)?;
        let surface_area = total_surface_area(&voxels);
        Ok(surface_area.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let voxels = parse_voxels(ctx)?;
        let surface_area = exterior_surface_area(&voxels);
        Ok(surface_area.to_string())
    }
}

fn total_surface_area(voxels: &Vec<Point3>) -> i32 {
    let mut area = 0;
    let mut voxel_set = HashSet::new();
    for voxel in voxels {
        if !voxel_set.insert(*voxel) {
            continue;
        }

        area += 6;
        for direction in DIRECTIONS {
            if voxel_set.contains(&(*voxel + direction)) {
                area -= 2;
            }
        }
    }

    area
}

fn exterior_surface_area(voxels: &Vec<Point3>) -> i32 {
    let mut lava_droplet = HashSet::new();
    let mut box_start = *voxels.get(0).unwrap_or(&Point3::default());
    let mut box_end = *voxels.get(0).unwrap_or(&Point3::default());
    for voxel in voxels {
        box_start = box_start.min_parts(*voxel - Point3::new(1, 1, 1));
        box_end = box_end.max_parts(*voxel + Point3::new(1, 1, 1));
        lava_droplet.insert(voxel);
    }

    let mut area = 0;
    let mut queue = VecDeque::from([box_start]);
    let mut visited = HashSet::from([box_start]);
    while let Some(current) = queue.pop_front() {
        for dir in DIRECTIONS {
            let neighbor = current + dir;
            if !is_in_bounds(&neighbor, &box_start, &box_end) {
                continue;
            }

            if lava_droplet.contains(&neighbor) {
                area += 1
            } else if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    area
}

fn is_in_bounds(point: &Point3, start: &Point3, end: &Point3) -> bool {
    point.x >= start.x
        && point.x <= end.x
        && point.y >= start.y
        && point.y <= end.y
        && point.z >= start.z
        && point.z <= end.z
}

fn parse_voxels(ctx: &Context) -> GenericResult<Vec<Point3>> {
    ctx.input()
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|c| c.parse().ok())
                .collect_tuple()
                .map(|(x, y, z)| Point3::new(x, y, z))
        })
        .collect::<Option<_>>()
        .ok_or("invalid input".into())
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Point3 {
    pub fn min_parts(&self, b: Point3) -> Point3 {
        Point3::new(self.x.min(b.x), self.y.min(b.y), self.z.min(b.z))
    }

    pub fn max_parts(&self, b: Point3) -> Point3 {
        Point3::new(self.x.max(b.x), self.y.max(b.y), self.z.max(b.z))
    }
}
