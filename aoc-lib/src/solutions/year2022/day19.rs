use crate::{helpers::re_capture_groups, solution::*, util::GenericResult};
use derive_more::Constructor;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

static GEO_IDX: usize = 3;

#[derive(Default)]
pub struct Day19;
impl Solution for Day19 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 19, "Not Enough Minerals")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let blueprints = parse_blueprints(ctx)?;
        let results = blueprints
            .iter()
            .enumerate()
            .map(|(index, bp)| {
                ctx.progress(index as f32 / blueprints.len() as f32);
                (bp.id, find_best(bp, ctx))
            })
            .collect_vec();
        println!("{:?}", results);

        let quality_level_sum = results.iter().fold(0, |a, (id, geo)| a + id * geo);
        Ok(quality_level_sum.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

#[derive(Clone, Constructor)]
struct State {
    remaining_time: i64,
    robots: Vec<i64>,
    ores: Vec<i64>,
}
impl State {
    fn hashed(self) -> (State, u128) {
        let mut hash = 0;
        self.robots
            .iter()
            .for_each(|c| hash = hash * 32 + *c as u128);
        self.ores
            .iter()
            .for_each(|c| hash = hash * 128 + *c as u128);
        (self, hash)
    }
}

fn find_best(bp: &Blueprint, _ctx: &Context) -> i64 {
    let (initial_state, initial_hash) = State::new(24, vec![1, 0, 0, 0], vec![0, 0, 0, 0]).hashed();
    let mut visited = HashSet::from([initial_hash]);
    let mut queue = VecDeque::from([initial_state]);
    let max_robot_counts = (0..3)
        .map(|ore_i| bp.costs.iter().fold(0, |a, costs| a.max(costs[ore_i])))
        .chain([i64::MAX])
        .collect_vec();

    let mut max = 0;
    let mut min_time = i64::MAX;
    while let Some(state) = queue.pop_front() {
        if state.remaining_time < min_time {
            min_time = state.remaining_time;
            // println!("Minute: {}", 25 - min_time);
        }

        let current_geo = state.ores[GEO_IDX];
        if current_geo > max {
            max = state.ores[GEO_IDX];
            // println!(
            //     "geo: {}, ores: {:?}, robots: {:?}",
            //     max, state.ores, state.robots
            // );
        }

        if state.remaining_time == 0 {
            continue;
        }

        let mut next_state = state.clone();
        next_state.remaining_time -= 1;
        for (robot_idx, robot_count) in state.robots.iter().enumerate() {
            next_state.ores[robot_idx] += robot_count;
        }

        let mut made_geo_robot = false;
        for robot_idx in (0..bp.costs.len()).rev() {
            let can_make_robot = state.robots[robot_idx] < max_robot_counts[robot_idx]
                && bp.costs[robot_idx]
                    .iter()
                    .enumerate()
                    .all(|(i, &c)| state.ores[i] >= c);
            if can_make_robot {
                let mut next_state = next_state.clone();
                next_state.robots[robot_idx] += 1;
                bp.costs[robot_idx]
                    .iter()
                    .enumerate()
                    .for_each(|(i, &c)| next_state.ores[i] -= c);

                let (next_state, hash) = next_state.hashed();
                if visited.insert(hash) {
                    queue.push_back(next_state);
                    if robot_idx == GEO_IDX {
                        made_geo_robot = true;
                    }
                } else {
                    // println!("dupl");
                }
            }
        }

        if !made_geo_robot {
            let (next_state, hash) = next_state.hashed();
            if visited.insert(hash) {
                queue.push_back(next_state);
            } else {
                // println!("dupl2");
            }
        }
    }

    max
}

// we want to build geode robots
// a, wait around if we got enough robots
// NEED D --------------------> NEED A
//      |--> NEED C ------------^^
//                |--> NEED B ---|
fn bfs_next_geo_robot(
    bp: &Blueprint,
    remaining_time: i64,
    robots: &Vec<i64>,
    ores: &Vec<i64>,
    cache: &mut HashMap<u128, Option<usize>>,
) -> Option<usize> {
    let hash = hash_robots_ores(robots, ores);
    if let Some(hit) = cache.get(&hash) {
        return hit.to_owned();
    }

    let initial_state = State::new(remaining_time, robots.clone(), ores.clone());
    let mut queue = VecDeque::from([(vec![], initial_state)]);
    let mut shortest_path = Vec::new();
    'bfs: while let Some((path, state)) = queue.pop_front() {
        if state.remaining_time == 0 {
            continue;
        }

        let mut next_state = state.clone();
        next_state.remaining_time -= 1;
        for (robot_idx, robot_count) in state.robots.iter().enumerate() {
            next_state.ores[robot_idx] += robot_count;
        }

        for robot_idx in 0..bp.costs.len() {
            let can_make_robot = bp.costs[robot_idx]
                .iter()
                .enumerate()
                .all(|(i, &c)| state.ores[i] >= c);
            if can_make_robot {
                if robot_idx == GEO_IDX {
                    shortest_path = path;
                    shortest_path.push(Some(robot_idx));
                    break 'bfs; // TODO might be more paths...
                }

                let mut next_path = path.clone();
                next_path.push(Some(robot_idx));

                let mut next_state = next_state.clone();
                next_state.robots[robot_idx] += 1;
                bp.costs[robot_idx]
                    .iter()
                    .enumerate()
                    .for_each(|(i, &c)| next_state.ores[i] -= c);

                queue.push_back((next_path, next_state));
            }
        }

        // wait around
        let mut next_path = path.clone();
        next_path.push(None);
        queue.push_back((next_path, next_state));
    }

    println!("{:?}", shortest_path);
    let result = shortest_path.into_iter().next().unwrap_or(None);
    cache.insert(hash, result.clone());

    result
}

fn hash_robots_ores(robots: &Vec<i64>, ores: &Vec<i64>) -> u128 {
    let mut hash = 0;
    robots.iter().for_each(|c| hash = hash * 32 + *c as u128);
    ores.iter().for_each(|c| hash = hash * 128 + *c as u128);
    hash
}

fn parse_blueprints(ctx: &Context) -> GenericResult<Vec<Blueprint>> {
    let blueprint_re_str = [
        r"(\d+):.*ore robot costs (\d+) ore.*",
        r"clay robot costs (\d+) ore.*",
        r"obsidian robot costs (\d+) ore and (\d+) clay.*",
        r"geode robot costs (\d+) ore and (\d+) obsidian",
    ]
    .join("");
    let blueprint_re = Regex::new(&blueprint_re_str)?;

    let mut blueprints = Vec::new();
    for line in ctx.input().lines() {
        let (id, ore_ore, cla_ore, obs_ore, obs_cla, geo_ore, geo_obs) =
            re_capture_groups(&blueprint_re, line)
                .ok_or("invalid_input")?
                .iter()
                .map(|p| p.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .collect_tuple()
                .ok_or("invalid_input")?;
        let costs = [
            [ore_ore, 0, 0],
            [cla_ore, 0, 0],
            [obs_ore, obs_cla, 0],
            [geo_ore, 0, geo_obs],
        ];
        blueprints.push(Blueprint { id, costs });
    }

    Ok(blueprints)
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Blueprint {
    id: i64,
    costs: [[i64; 3]; 4],
}
