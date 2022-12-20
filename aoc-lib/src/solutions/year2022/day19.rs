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
                (bp.id, find_best(bp, 24, ctx, index, blueprints.len()))
            })
            .collect_vec();

        let quality_level_sum = results.iter().fold(0, |a, (id, geo)| a + id * geo);
        Ok(quality_level_sum.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let blueprints = parse_blueprints(ctx)?.into_iter().take(3).collect_vec();
        let results = blueprints
            .iter()
            .enumerate()
            .map(|(index, bp)| {
                ctx.progress(index as f32 / blueprints.len() as f32);
                find_best(bp, 32, ctx, index, blueprints.len())
            })
            .collect_vec();

        let product = results.iter().fold(1, |a, x| a * x);
        Ok(product.to_string()) // 31 not good
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

fn get_max_potential(mut robot_count: i64, mut geo_count: i64, remaining_time: i64) -> i64 {
    for _ in 0..remaining_time {
        geo_count += robot_count;
        robot_count += 1;
    }
    geo_count
}

fn find_best(
    bp: &Blueprint,
    available_time: i64,
    ctx: &Context,
    bp_idx: usize,
    bp_count: usize,
) -> i64 {
    let (initial_state, initial_hash) =
        State::new(available_time, vec![1, 0, 0, 0], vec![0, 0, 0, 0]).hashed();
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

            ctx.progress(
                bp_idx as f32 / bp_count as f32
                    + (1.0 / bp_count as f32) * (available_time - state.remaining_time) as f32
                        / available_time as f32,
            )
        }

        let current_geo = state.ores[GEO_IDX];
        if current_geo > max {
            max = state.ores[GEO_IDX];
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
                }
            }
        }

        if !made_geo_robot {
            let (next_state, hash) = next_state.hashed();
            if visited.insert(hash) {
                queue.push_back(next_state);
            }
        }
    }

    max
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
