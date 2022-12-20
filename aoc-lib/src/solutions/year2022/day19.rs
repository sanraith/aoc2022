use crate::{helpers::re_capture_groups, solution::*, util::GenericResult};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, hash::Hash};

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
                (bp.id, start_find_best_rec(bp, 24))
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
                // find_best(bp, 32, ctx, index, blueprints.len())
                start_find_best_rec(bp, 32)
            })
            .collect_vec();
        println!("{:?}", results);

        let product = results.iter().fold(1, |a, x| a * x);
        Ok(product.to_string()) // 31 not good
    }
}

fn hash_state(robots: &Vec<i64>, ores: &Vec<i64>) -> u128 {
    let mut hash: u128 = 0;
    // robots.iter().for_each(|c| hash = hash * 32 + *c as u128);
    // ores.iter().for_each(|c| hash = hash * 128 + *c as u128);
    robots.iter().for_each(|c| {
        hash = hash
            .checked_mul(32)
            .unwrap()
            .checked_add(*c as u128)
            .unwrap()
    });
    ores.iter().for_each(|c| {
        hash = hash
            .checked_mul(128)
            .unwrap()
            .checked_add(*c as u128)
            .unwrap()
    });

    hash
}

fn start_find_best_rec(bp: &Blueprint, remaining_time: i64) -> i64 {
    let max_robot_counts = (0..3)
        .map(|ore_i| bp.costs.iter().fold(0, |a, costs| a.max(costs[ore_i])))
        .chain([i64::MAX])
        .collect_vec();

    find_best_rec(
        bp,
        remaining_time,
        &mut vec![0, 0, 0, 0],
        &mut vec![1, 0, 0, 0],
        &mut HashSet::new(),
        &max_robot_counts,
    )
}

fn find_best_rec(
    bp: &Blueprint,
    remaining_time: i64,
    ores: &mut Vec<i64>,
    robots: &mut Vec<i64>,
    visited: &mut HashSet<u128>,
    max_robot_counts: &Vec<i64>,
) -> i64 {
    let mut possible_robots = Vec::new();
    let mut made_geo_robot = false;
    if remaining_time > 1 {
        for (robot_idx, resources) in bp.costs.iter().enumerate().rev() {
            let can_make_robot = robots[robot_idx] < max_robot_counts[robot_idx]
                && resources.iter().enumerate().all(|(i, &c)| ores[i] >= c);

            if can_make_robot {
                possible_robots.push(robot_idx);
                if robot_idx == GEO_IDX {
                    made_geo_robot = true;
                }
            }
        }
    }

    // produce ores
    for (robot_type, count) in robots.iter().enumerate() {
        ores[robot_type] += count;
    }

    let mut max_geo = ores[GEO_IDX];
    if remaining_time > 0 {
        // make robots
        for (robot_type, resources) in possible_robots.iter().map(|&t| (t, bp.costs[t])) {
            resources.iter().enumerate().for_each(|(i, c)| ores[i] -= c);
            robots[robot_type] += 1;

            let hash = hash_state(robots, ores);
            if visited.insert(hash) {
                let count = find_best_rec(
                    bp,
                    remaining_time - 1,
                    ores,
                    robots,
                    visited,
                    max_robot_counts,
                );
                max_geo = max_geo.max(count);
            }

            robots[robot_type] -= 1;
            resources.iter().enumerate().for_each(|(i, c)| ores[i] += c);
        }

        // or pass time
        if !made_geo_robot {
            let hash = hash_state(robots, ores);
            if visited.insert(hash) {
                let count = find_best_rec(
                    bp,
                    remaining_time - 1,
                    ores,
                    robots,
                    visited,
                    max_robot_counts,
                );
                max_geo = max_geo.max(count);
            }
        }
    }

    // clean up since we are using the same vectors
    for (robot_type, count) in robots.iter().enumerate() {
        ores[robot_type] -= count;
    }

    return max_geo;
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
