use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Mul, MulAssign, Sub, SubAssign};
use itertools::Itertools;
use num::integer::Roots;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::RangeInclusive,
};

static TILE_OPEN: char = '.';
static TILE_WALL: char = '#';
static DIRECTIONS: [Point; 4] = [
    // R, D, L, U
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

static DIRECTIONS3: [Point3; 4] = [
    // R, D, L, U; y is flipped
    Point3 { x: 1, y: 0, z: 0 },
    Point3 { x: 0, y: -1, z: 0 },
    Point3 { x: -1, y: 0, z: 0 },
    Point3 { x: 0, y: 1, z: 0 },
];

#[derive(Default)]
pub struct Day22;
impl Solution for Day22 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 22, "Monkey Map")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_input(ctx, WrapMode::Flat)?;
        let (pos, facing) = walk(&map)?;
        let password = (pos.y + 1) * 1000 + (pos.x + 1) * 4 + facing as i32;

        Ok(password.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let map = parse_input(ctx, WrapMode::Cube)?;
        let (pos, facing) = walk_cube(&map, ctx)?;
        let password = (pos.y + 1) * 1000 + (pos.x + 1) * 4 + facing as i32;

        Ok(password.to_string())
    }
}

fn walk_cube(map: &Map, ctx: &Context) -> GenericResult<(Point, usize)> {
    let mut cube = map.cube.clone();
    let mut pos3 = cube
        .get_3d_pos(&map.start.clone())
        .ok_or("invalid 3d map")?;
    let mut facing3 = 0;
    let mut visited = Vec::new();

    for (index, (facing_next, distance)) in map.path.iter().enumerate() {
        ctx.progress(index as f32 / map.path.len() as f32);

        facing3 = (facing3 + *facing_next) % 4;
        let dir3 = DIRECTIONS3[facing3];

        for _ in 0..*distance {
            let cube_orig = cube.clone();
            let mut next_pos3 = pos3 + dir3;

            match next_pos3 {
                p if p.x < 0 => {
                    cube.rotate(rad(90), 0.0, 0.0);
                    next_pos3.x = map.side_len - 1;
                }
                p if p.x >= map.side_len => {
                    cube.rotate(rad(-90), 0.0, 0.0);
                    next_pos3.x = 0;
                }
                p if p.y < 0 => {
                    cube.rotate(0.0, rad(-90), 0.0);
                    next_pos3.y = map.side_len - 1;
                }
                p if p.y >= map.side_len => {
                    cube.rotate(0.0, rad(90), 0.0);
                    next_pos3.y = 0;
                }
                _ => (),
            };

            let pos2 = *cube.tiles.get(&next_pos3).ok_or("invalid 3d map")?;
            if *map.tiles.get(&pos2).ok_or("invalid 3d map")? == TILE_WALL {
                cube = cube_orig;
                break;
            } else {
                visited.push(pos2);
                pos3 = next_pos3;
            }
        }
    }

    let pos2 = visited[visited.len() - 1];
    let facing = DIRECTIONS
        .iter()
        .position(|&p| p == (pos2 - visited[visited.len() - 2]))
        .ok_or("case when turning on the last tile is not handled")?;

    Ok((pos2, facing))
}

fn walk(map: &Map) -> GenericResult<(Point, usize)> {
    let mut pos = map.start.clone();
    let mut facing = 0;
    let mut visited = HashMap::new();
    for (facing_next, distance) in &map.path {
        facing = (facing + *facing_next) % 4;
        for _ in 0..*distance {
            let (next_pos, facing_change) = get_next_pos(pos, facing, map)?;
            match map.tiles.get(&next_pos) {
                Some(&c) if c == TILE_WALL => break,
                Some(_) => {
                    visited.insert(pos, facing);
                    pos = next_pos;
                    facing = (facing as i32 + facing_change + 4) as usize % 4;
                }
                None => Err("should not arrive to void")?,
            }
        }
    }
    visited.insert(pos, facing);

    Ok((pos, facing))
}

fn get_next_pos(start: Point, facing: usize, map: &Map) -> GenericResult<(Point, i32)> {
    let pos = start + DIRECTIONS[facing];
    let (next_pos, facing_change) = match facing {
        0 | 2 => *map.portals_h.get(&pos).unwrap_or(&(pos, 0)),
        1 | 3 => *map.portals_v.get(&pos).unwrap_or(&(pos, 0)),
        _ => Err("invalid facing")?,
    };

    Ok((next_pos, facing_change))
}

#[allow(dead_code)]
fn print_map(map: &Map, visited: &HashMap<Point, usize>) {
    println!("\nmap:");
    for y in 0..map.height {
        for x in 0..map.width {
            let p = Point::new(x, y);
            if let Some(facing) = visited.get(&p) {
                match facing {
                    0 => print!(">"),
                    1 => print!("v"),
                    2 => print!("<"),
                    3 => print!("^"),
                    _ => print!("?"),
                }
            } else {
                match map.tiles.get(&p) {
                    Some(c) => print!("{}", c),
                    None => print!(" "),
                }
            }
        }
        println!();
    }
}

fn parse_input(ctx: &Context, mode: WrapMode) -> GenericResult<Map> {
    let input = ctx.input();
    let (input_map, input_path) = input.split_once("\n\n").ok_or("invalid input")?;

    let mut tiles = HashMap::new();
    let mut ranges_x: HashMap<i32, RangeInclusive<i32>> = HashMap::new();
    let mut ranges_y: HashMap<i32, RangeInclusive<i32>> = HashMap::new();
    let mut start_x = None;
    for (y, line) in input_map.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if c != TILE_OPEN && c != TILE_WALL {
                continue;
            }
            if y == 0 && start_x.is_none() && c == TILE_OPEN {
                start_x = Some(x);
            }

            tiles.insert(Point::new(x as i32, y as i32), c);
            ranges_x
                .entry(y)
                .and_modify(|r| *r = *r.start().min(&x)..=*r.end().max(&x))
                .or_insert(x..=x);
            ranges_y
                .entry(x)
                .and_modify(|r| *r = *r.start().min(&y)..=*r.end().max(&y))
                .or_insert(y..=y);
        }
    }
    let err = "map is empty";
    let width = ranges_x.iter().map(|(_, r)| *r.end()).max().ok_or(err)? + 1;
    let height = ranges_y.iter().map(|(_, r)| *r.end()).max().ok_or(err)? + 1;
    let side_len = (tiles.len() / 6).sqrt() as i32;

    let mut facing = 0;
    let mut path = Vec::new();
    let path_part_re = Regex::new(r"(-?\d+)([LR])?")?;
    for captures in path_part_re.captures_iter(input_path) {
        let distance = captures
            .get(1)
            .and_then(|x| x.as_str().parse::<i32>().ok())
            .ok_or("invalid path")?;
        path.push((facing, distance));

        let turn = captures.get(2).and_then(|x| x.as_str().chars().next());
        match turn {
            Some(c) if c == 'R' => facing = 1,
            Some(c) if c == 'L' => facing = 3,
            Some(_) => Err("invalid path")?,
            None => (),
        }
    }

    let mut map = Map {
        start: Point::new(start_x.ok_or("could not find start position")?, 0),
        tiles,
        path,
        width,
        height,
        side_len,
        ..Default::default()
    };
    match mode {
        WrapMode::Flat => add_flat_portals(&mut map, &ranges_x, &ranges_y),
        WrapMode::Cube => add_cube(&mut map)?,
    };

    Ok(map)
}

fn add_flat_portals(
    map: &mut Map,
    ranges_x: &HashMap<i32, RangeInclusive<i32>>,
    ranges_y: &HashMap<i32, RangeInclusive<i32>>,
) {
    let portals_h = &mut map.portals_h;
    for (y, r) in ranges_x {
        portals_h.insert(Point::new(r.start() - 1, *y), (Point::new(*r.end(), *y), 0));
        portals_h.insert(Point::new(r.end() + 1, *y), (Point::new(*r.start(), *y), 0));
    }
    let portals_v = &mut map.portals_v;
    for (x, r) in ranges_y {
        portals_v.insert(Point::new(*x, r.start() - 1), (Point::new(*x, *r.end()), 0));
        portals_v.insert(Point::new(*x, r.end() + 1), (Point::new(*x, *r.start()), 0));
    }
}

fn add_cube(map: &mut Map) -> GenericResult {
    // find position of sides within the tile map
    let mut sides = Vec::new();
    for y in 0..(map.height / map.side_len) {
        for x in 0..(map.width / map.side_len) {
            let tile_pos = Point::new(x * map.side_len, y * map.side_len);
            if map.tiles.contains_key(&tile_pos) {
                sides.push(Point::new(x, y));
            }
        }
    }

    // map sides to cube
    let start = sides[0].clone();
    let sides_map: HashSet<Point> = HashSet::from_iter(sides);
    let mut cube = Cube {
        side_len: map.side_len,
        ..Default::default()
    };
    fill_cube(&start, &sides_map, &mut HashSet::new(), &mut cube, &map);

    map.cube = cube;
    Ok(())
}

fn fill_cube(
    side: &Point,
    sides: &HashSet<Point>,
    visited: &mut HashSet<Point>,
    cube: &mut Cube,
    map: &Map,
) {
    for y in side.y * cube.side_len..side.y * cube.side_len + cube.side_len {
        for x in side.x * cube.side_len..side.x * cube.side_len + cube.side_len {
            let from = Point::new(x, y);
            let to = Point3::new(
                x - side.x * cube.side_len,
                map.side_len - 1 - (y - side.y * cube.side_len), // flip y
                1,
            );
            cube.tiles.insert(to, from);
        }
    }

    for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
        let next_side = *side + *dir;
        if visited.insert(next_side) && sides.contains(&next_side) {
            match dir_idx {
                0 | 2 => cube.rotate(rad((dir_idx as i32 - 1) * 90), 0.0, 0.0),
                _ => cube.rotate(0.0, rad((dir_idx as i32 - 2) * 90), 0.0),
            }
            fill_cube(&next_side, sides, visited, cube, map);
            match dir_idx {
                0 | 2 => cube.rotate(rad((dir_idx as i32 - 1) * -90), 0.0, 0.0),
                _ => cube.rotate(0.0, rad((dir_idx as i32 - 2) * -90), 0.0),
            }
        }
    }
}

#[derive(Debug, Default)]
struct Map {
    start: Point,
    cube: Cube,
    tiles: HashMap<Point, char>,
    width: i32,
    height: i32,
    side_len: i32,
    path: Vec<(usize, i32)>,
    portals_h: HashMap<Point, (Point, i32)>,
    portals_v: HashMap<Point, (Point, i32)>,
}

#[derive(Debug, Default, Clone)]
struct Cube {
    side_len: i32,
    tiles: HashMap<Point3, Point>,
}
impl Cube {
    fn get_3d_pos(&self, p: &Point) -> Option<Point3> {
        self.tiles.iter().find(|(_, &p2)| p2 == *p).map(|x| *x.0)
    }

    #[allow(non_snake_case)]
    fn rotate(&mut self, pitch: f32, roll: f32, yaw: f32) {
        let cosa = yaw.cos();
        let sina = yaw.sin();
        let cosb = pitch.cos();
        let sinb = pitch.sin();
        let cosc = roll.cos();
        let sinc = roll.sin();
        let Axx = cosa * cosb;
        let Axy = cosa * sinb * sinc - sina * cosc;
        let Axz = cosa * sinb * cosc + sina * sinc;
        let Ayx = sina * cosb;
        let Ayy = sina * sinb * sinc + cosa * cosc;
        let Ayz = sina * sinb * cosc - cosa * sinc;
        let Azx = -sinb;
        let Azy = cosb * sinc;
        let Azz = cosb * cosc;

        let t = Point3F::new(
            (self.side_len as f32 - 1.0) / -2.0,
            (self.side_len as f32 - 1.0) / -2.0,
            (self.side_len as f32 - 1.0) / 2.0,
        );

        // translate
        let tiles_f = self.tiles.drain().map(|(p, c)| (Point3F::from(p) + t, c));

        // rotate
        let tiles_f = tiles_f.map(|(p, c)| {
            let p2 = Point3F::new(
                Axx * p.x + Axy * p.y + Axz * p.z,
                Ayx * p.x + Ayy * p.y + Ayz * p.z,
                Azx * p.x + Azy * p.y + Azz * p.z,
            );
            (p2, c)
        });

        // translate back
        self.tiles = HashMap::from_iter(tiles_f.map(|(p, c)| (Point3::from(p - t), c)));
    }

    #[allow(dead_code)]
    fn print_coords(&self, map: &Map, tile: char) {
        let coords = self
            .tiles
            .iter()
            .filter(|(_, p2)| *map.tiles.get(&p2).unwrap_or(&TILE_OPEN) == tile)
            .map(|(p1, _)| p1)
            .collect_vec();
        println!("x=[{}]", coords.iter().map(|p| p.x).join(", "));
        println!("y=[{}]", coords.iter().map(|p| p.y).join(", "));
        println!("z=[{}]", coords.iter().map(|p| p.z).join(", "));
    }
}

fn rad(deg: i32) -> f32 {
    (deg as f32).to_radians()
}

enum WrapMode {
    Flat,
    Cube,
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, Constructor, Add, Sub, AddAssign, SubAssign,
)]
struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl From<Point3F> for Point3 {
    fn from(p: Point3F) -> Self {
        Point3::new(p.x.round() as i32, p.y.round() as i32, p.z.round() as i32)
    }
}

#[derive(
    Copy, Clone, Debug, Default, Constructor, Add, Sub, Mul, AddAssign, SubAssign, MulAssign,
)]
struct Point3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl From<Point3> for Point3F {
    fn from(p: Point3) -> Self {
        Point3F::new(p.x as f32, p.y as f32, p.z as f32)
    }
}
