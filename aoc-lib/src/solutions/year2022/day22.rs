use crate::{solution::*, util::GenericResult};
use derive_more::{Add, AddAssign, Constructor, Mul, MulAssign, Sub, SubAssign};
use itertools::Itertools;
use num::integer::Roots;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::{Range, RangeInclusive},
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

// side => [(neighbor_side, neighbor_side_rotation); 4] in the order of [R, D, L, U]
static DICE: Lazy<HashMap<usize, [(usize, usize); 4]>> = Lazy::new(|| {
    HashMap::from([
        (1, [(3, 3), (5, 0), (4, 1), (2, 2)]),
        (2, [(4, 0), (6, 2), (3, 0), (1, 2)]),
        (3, [(2, 0), (6, 3), (5, 0), (1, 1)]),
        (4, [(5, 0), (6, 1), (2, 0), (1, 3)]),
        (5, [(3, 0), (6, 0), (4, 0), (1, 0)]),
        (6, [(3, 1), (2, 2), (4, 3), (5, 0)]),
    ])
});

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
        let (pos, facing) = walk(&map)?;
        let password = (pos.y + 1) * 1000 + (pos.x + 1) * 4 + facing as i32;

        Ok(password.to_string())
    }
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

    print_map(map, &visited);

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
                    _ => print!("^"),
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
        WrapMode::Cube => add_cube_portals(&mut map)?,
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

fn add_cube_portals(map: &mut Map) -> GenericResult {
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

    // identify sides by their number on the dice and save 0 rotation connections
    let start = sides.iter().next().ok_or("invalid input")?;
    let mut queue = VecDeque::from([(*start, (1, 0))]);
    let mut side_map = HashMap::from([(*start, (1, 0))]);
    let mut side_rotation_map: HashMap<(usize, usize), i32> = HashMap::new();
    while let Some((pos, (side, rot))) = queue.pop_front() {
        let neighbors = DICE.get(&side).ok_or("invalid dice")?;
        for (dir_idx, (next_side, next_rot)) in neighbors.iter().enumerate() {
            let next_pos = pos + DIRECTIONS[(dir_idx + rot) % 4];
            if sides.contains(&next_pos) && !side_map.contains_key(&next_pos) {
                side_map.insert(next_pos, (*next_side, *next_rot));
                queue.push_back((next_pos, (*next_side, *next_rot)));
                side_rotation_map.insert((side, *next_side), 0);
                side_rotation_map.insert((*next_side, side), 0);
            }
        }
    }

    // find out relative rotations
    for (a, b) in itertools::iproduct!(1..=6, 1..=6).filter(|(a, b)| a != b) {
        if side_rotation_map.contains_key(&(a, b)) {
            continue; // already covered
        }
        if DICE
            .get(&a)
            .ok_or("err")?
            .iter()
            .all(|(side, _)| *side != b)
        {
            continue; // not neighbors
        }

        // println!("{} {}", a, b);
        let pos_a = side_map
            .iter()
            .find(|(_, (s, _))| *s == a)
            .map(|(k, _)| k)
            .ok_or("err")?;
        let pos_b = side_map
            .iter()
            .find(|(_, (s, _))| *s == b)
            .map(|(k, _)| k)
            .ok_or("err")?;

        let rotation = get_relative_rotation(&sides, &pos_a, &pos_b); // (400 + (pos_a.manhattan(pos_b) - 1) * (pos_a.x - pos_b.x).signum()) % 4;
        side_rotation_map.insert((a, b), rotation);
    }

    println!("\nCube shape:");
    for y in 0..4 {
        for x in 0..4 {
            let c = match side_map.get(&Point::new(x, y)) {
                Some((side, _rot)) => side.to_string(),
                None => " ".to_owned(),
            };
            print!("{}", c)
        }
        println!();
    }

    // generate portals based on relative rotations
    for (pos_a, (side_a, _)) in side_map.iter().collect_vec() {
        for (dir_idx, (side_b, _)) in DICE.get(&side_a).ok_or("err")?.iter().enumerate() {
            let pos_b = side_map
                .iter()
                .find(|(k, (s, _))| s == side_b)
                .map(|(k, _)| k)
                .ok_or("err")?;

            let rot = (4 + get_relative_rotation(&sides, &pos_a, &pos_b)) % 4;
            // ((400 + (pos_a.manhattan(pos_b) - 1) * (pos_a.x - pos_b.x).signum()) % 4) as usize;
            let edge_from = dir_idx;
            let edge_to = (dir_idx as i32 + 2 + rot) as usize % 4;

            let points_a = edge_ranges(map, &pos_a, edge_from)?;
            let points_a = itertools::iproduct!(points_a.0, points_a.1)
                .map(|(x, y)| Point::new(x, y))
                .collect_vec();
            let points_b = edge_ranges(map, &pos_b, edge_to)?;
            let mut points_b = itertools::iproduct!(points_b.0, points_b.1)
                .map(|(x, y)| Point::new(x, y))
                .collect_vec();

            // if *side_a == 4 && *side_b == 1 {
            //     println!(
            //         "\n{} {} {}",
            //         (pos_a.manhattan(pos_b) - 1),
            //         (pos_b.x - pos_a.x).signum(),
            //         ((400 + (pos_a.manhattan(pos_b) - 1) * (pos_b.x - pos_a.x).signum()) % 4)
            //     );
            //     println!("{}", rot);
            //     println!("{:?}", points_b);
            // }

            // match (edge_from, edge_to) {
            //     // (0, 0) => points_b.reverse(),
            //     (0, 1) => points_b.reverse(),
            //     (0, 2) => (),
            //     // (0, 3) => points_b.reverse(),
            //     // (1, 0) => (),
            //     // (1, 1) => points_b.reverse(),
            //     (1, 2) => points_b.reverse(),
            //     (1, 3) => (),
            //     // (2, 0) => (),
            //     // (2, 1) => (),
            //     // (2, 2) => (),
            //     // (2, 3) => (),
            //     // (3, 0) => (),
            //     // (3, 1) => (),
            //     // (3, 2) => (),
            //     // (3, 3) => (),

            //     // TODO
            //     _ => Err(format!(
            //         "i don't know e{}->{} s{}->{}",
            //         edge_from, edge_to, side_a, side_b
            //     ))?,
            // };

            // if points_a[0].y < points_b[0].y {
            //     points_b.reverse();
            // }
            match rot {
                0 | 3 => (),
                2 | 1 => points_b.reverse(),
                _ => Err("invalid rotation")?,
            }
            // if *side_a == 4 && *side_b == 1 {
            //     println!("{:?}", points_b);
            // }

            for (idx, point_a) in points_a.into_iter().enumerate() {
                let point_a = point_a + DIRECTIONS[dir_idx];
                if map.tiles.get(&point_a).is_some() {
                    continue;
                }

                let point_b = points_b[idx];
                // if *side_a == 4 && *side_b == 1 {
                //     println!(" {:?}->{:?}", point_a, point_b);
                // }
                let prev = match dir_idx {
                    0 | 2 => map.portals_h.insert(point_a, (point_b, rot as i32)),
                    1 | 3 => map.portals_v.insert(point_a, (point_b, rot as i32)),
                    _ => Err("invalid direction")?,
                };
                if prev.is_some() {
                    Err("not good")?;
                }
            }
        }
    }

    println!(
        "{:#?}",
        side_rotation_map
            .iter()
            .map(|((a, b), r)| format!("{}->{} rot: {}", a, b, r))
            .collect_vec()
    );

    // println!("\nmap:");
    // for y in -1..map.height + 1 {
    //     for x in -1..map.width + 1 {
    //         let p = Point::new(x, y);

    //         match map.tiles.get(&p) {
    //             Some(c) => print!("{}", c),
    //             None => print!(" "),
    //         }
    //     }
    //     println!();
    // }

    // println!("\nmap with portals:");
    // for y in -1..map.height + 1 {
    //     for x in -1..map.width + 1 {
    //         let p = Point::new(x, y);
    //         let mut portal_count = if map.portals_h.contains_key(&p) { 1 } else { 0 };
    //         portal_count += if map.portals_v.contains_key(&p) { 1 } else { 0 };
    //         if portal_count > 0 {
    //             print!("{}", portal_count);
    //         } else {
    //             match map.tiles.get(&p) {
    //                 Some(c) => print!("{}", c),
    //                 None => print!(" "),
    //             }
    //         }
    //     }
    //     println!();
    // }

    // println!(
    //     "{:#?}",
    //     side_map
    //         .iter()
    //         .map(|(p, (a, b))| format!("({}, {}) side: {}, rot: {}", p.x, p.y, a, b))
    //         .collect_vec()
    // );
    // println!(
    //     "{:#?}",
    //     side_rotation_map
    //         .iter()
    //         .map(|((a, b), v)| format!("{}->{}: {}", a, b, v))
    //         .collect_vec()
    // );

    Ok(())
}

fn get_relative_rotation(sides: &Vec<Point>, a: &Point, b: &Point) -> i32 {
    let sides: HashSet<Point> = HashSet::from_iter(sides.clone());
    let mut hor: i32 = 0;
    let mut ver: i32 = 0;
    let mut visited = HashSet::from([*a]);
    let mut queue = VecDeque::from([(*a, 0, 0)]);

    // let rotation_by_side

    while let Some((p, _hor, _ver)) = queue.pop_front() {
        if p == *b {
            hor = _hor;
            ver = _ver;
            break;
        }

        for (dir_idx, dir) in DIRECTIONS.iter().enumerate() {
            let mut _hor = _hor;
            let mut _ver = _ver;
            let next = p + *dir;
            match dir_idx {
                0 | 2 => _hor += (dir_idx as i32 - 1) * 90,
                1 | 3 => _ver += (dir_idx as i32 - 2) * 90,
                _ => (),
            };

            if sides.contains(&next) && visited.insert(next) {
                queue.push_back((next, _hor, _ver));
            }
        }
    }

    ver + hor
}

fn edge_ranges(
    map: &Map,
    side_pos: &Point,
    edge: usize,
) -> GenericResult<(Range<i32>, Range<i32>)> {
    let p = *side_pos * map.side_len;
    let n = map.side_len;
    let (range_x, range_y) = match edge {
        0 => (p.x + n - 1..p.x + n, p.y..p.y + n),
        1 => (p.x..p.x + n, p.y + n - 1..p.y + n),
        2 => (p.x..p.x + 1, p.y..p.y + n),
        3 => (p.x..p.x + n, p.y..p.y + 1),
        _ => Err("invalid edge")?,
    };

    Ok((range_x, range_y))
}

#[derive(Debug, Default)]
struct Map {
    start: Point,
    tiles: HashMap<Point, char>,
    width: i32,
    height: i32,
    side_len: i32,
    path: Vec<(usize, i32)>,
    portals_h: HashMap<Point, (Point, i32)>,
    portals_v: HashMap<Point, (Point, i32)>,
}

enum WrapMode {
    Flat,
    Cube,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    Hash,
    PartialEq,
    Eq,
    Constructor,
    Add,
    Sub,
    Mul,
    AddAssign,
    SubAssign,
    MulAssign,
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
