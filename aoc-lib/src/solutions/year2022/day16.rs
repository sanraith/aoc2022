use crate::{
    helpers::{is_wasm, re_capture_groups},
    solution::*,
    util::GenericResult,
};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    sync::Mutex,
};

#[derive(Default)]
pub struct Day16;
impl Solution for Day16 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 16, "Proboscidea Volcanium")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let network = parse_network(ctx)?;
        let valve_states = &mut &mut vec![false; network.valves.len()];
        let (released_pressure, ..) =
            release_pressure(&network, network.start, valve_states, 30, 0, 0, ctx, 0);

        Ok(released_pressure.to_string())
    }

    // This solution is highly inefficient, runs for ~3 minutes on a i7-13700K
    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let network = parse_network(ctx)?;
        let agents = vec![
            AgentTask::Ready(network.start),
            AgentTask::Ready(network.start),
        ];
        let valve_states = &mut vec![false; network.valves.len()];
        let released_pressure =
            release_pressure_part2(&network, agents, valve_states, 26, 0, 0, ctx);

        Ok(released_pressure.to_string())
    }
}

fn release_pressure_part2(
    network: &Network,
    agents: Vec<AgentTask>,
    mut valve_states: &mut Vec<bool>,
    time_remaining: i32,
    total_pressure: i32,
    mut ppm: i32,
    ctx: &Context,
) -> i32 {
    // perform agent actions
    let mut restore_valve = Vec::new();
    let agents = agents
        .into_iter()
        .map(|a| match a {
            AgentTask::Ready(_) => a,
            AgentTask::Open(v, 0) => {
                valve_states[v] = true;
                restore_valve.push(v);
                ppm += network.valves[v].flow_rate;
                AgentTask::Ready(v)
            }
            AgentTask::Open(v, d) => AgentTask::Open(v, d - 1),
        })
        .collect_vec();

    // collect all valid actions for each agent regardless of each other
    let mut next_actions: Vec<Vec<AgentTask>> = vec![vec![]; agents.len()];
    for index in 0..agents.len() {
        let agent = &agents[index];
        match agent {
            AgentTask::Open(v, d) => next_actions[index].push(AgentTask::Open(*v, *d)),
            AgentTask::Ready(_) => {
                next_actions[index].append(&mut find_valid_tasks(network, &valve_states, agent))
            }
        };
    }

    // collect task pairs for each agent
    let mut tasks = Vec::new();
    if time_remaining > 1 {
        for a1 in &next_actions[0] {
            for a2 in &next_actions[1] {
                match (a1, a2) {
                    (AgentTask::Open(v1, _), AgentTask::Open(v2, _)) if v1 == v2 => continue,
                    (AgentTask::Ready(_), AgentTask::Ready(_)) => continue,
                    (AgentTask::Open(_, _), AgentTask::Ready(_))
                        if next_actions[1].len() > 0 && tasks.len() > 0 =>
                    {
                        continue;
                    }
                    (AgentTask::Ready(_), AgentTask::Open(_, _))
                        if next_actions[0].len() > 0 && tasks.len() > 0 =>
                    {
                        continue;
                    }
                    _ => tasks.push(vec![a1.clone(), a2.clone()]),
                }
            }
        }
    }

    // run via rayon at the top level if we are not in a wasm environment
    let mut max_tp = total_pressure + time_remaining * ppm;
    let tasks_len = tasks.len() as f32;
    let tp = match !is_wasm() && time_remaining > 25 {
        true => {
            let ctx_mutex = Mutex::new(ctx);
            tasks
                .into_iter()
                .enumerate()
                .par_bridge()
                .map(|(index, task)| {
                    if time_remaining == 26 {
                        ctx_mutex.lock().unwrap().progress(index as f32 / tasks_len);
                    }
                    release_pressure_part2(
                        network,
                        task,
                        &mut valve_states.clone(),
                        time_remaining - 1,
                        total_pressure + ppm,
                        ppm,
                        ctx,
                    )
                })
                .max()
        }
        false => tasks
            .into_iter()
            .enumerate()
            .map(|(index, task)| {
                if time_remaining == 26 {
                    ctx.progress(index as f32 / tasks_len);
                }
                release_pressure_part2(
                    network,
                    task,
                    &mut valve_states,
                    time_remaining - 1,
                    total_pressure + ppm,
                    ppm,
                    ctx,
                )
            })
            .max(),
    };
    tp.map(|x| max_tp = max_tp.max(x));

    restore_valve.iter().for_each(|&v| valve_states[v] = false);
    return max_tp;
}

fn release_pressure(
    network: &Network,
    current: usize,
    valves_open: &mut Vec<bool>,
    time_remaining: i32,
    total_pressure: i32,
    ppm: i32,
    ctx: &Context,
    level: i32,
) -> (i32, Vec<bool>) {
    if time_remaining <= 0 {
        return (total_pressure, valves_open.clone());
    }

    // released pressure by waiting around
    let mut max_tp = total_pressure + time_remaining * ppm;
    let mut max_state: Option<Vec<bool>> = None;

    let should_open_valve = !valves_open[current] && network.valves[current].flow_rate > 0;
    if should_open_valve {
        // open current valve if there is a point in doing it
        valves_open[current] = true;
        let (tp, state) = release_pressure(
            network,
            current,
            valves_open,
            time_remaining - 1,
            total_pressure + 1 * ppm,
            ppm + network.valves[current].flow_rate,
            ctx,
            level + 1,
        );
        if tp > max_tp {
            max_tp = tp;
            max_state = Some(state);
        }
        max_tp = max_tp.max(tp);
        valves_open[current] = false;
    }

    if !should_open_valve || level == 0 {
        for (to, distance) in network.distance_map[current]
            .iter()
            .enumerate()
            .filter(|(_, d)| d.is_some())
        {
            let distance = distance.unwrap();
            if distance > time_remaining || valves_open[to] || network.valves[to].flow_rate <= 0 {
                continue;
            }

            let (tp, state) = release_pressure(
                network,
                to,
                valves_open,
                time_remaining - distance,
                total_pressure + distance * ppm,
                ppm,
                ctx,
                level + 1,
            );
            if tp > max_tp {
                max_tp = tp;
                max_state = Some(state);
            }
        }
    }

    let max_state = match max_state {
        Some(max_state) => max_state,
        None => valves_open.clone(),
    };

    return (max_tp, max_state);
}

fn find_valid_tasks(
    network: &Network,
    valve_states: &Vec<bool>,
    agent: &AgentTask,
) -> Vec<AgentTask> {
    let mut tasks = Vec::new();
    let current_valve = match agent {
        AgentTask::Ready(v) => *v,
        AgentTask::Open(_, _) => panic!("logic error"),
    };

    for target_valve in
        (0..network.valves.len()).filter(|&v| !valve_states[v] && network.valves[v].flow_rate > 0)
    {
        if let Some(distance) = network.distance_map[current_valve][target_valve] {
            tasks.push(AgentTask::Open(target_valve, distance))
        }
    }
    tasks.push(AgentTask::Ready(current_valve));

    tasks
}

fn parse_network(ctx: &Context) -> GenericResult<Network> {
    let re = Regex::new(r"Valve (\S+) has flow rate=(-?\d+); tunnels? leads? to valves? (.*)")?;
    let mut network = Network::default();
    for line in ctx.input().lines() {
        let (name, flow_rate, pipes) = re_capture_groups(&re, line)
            .and_then(|x| x.into_iter().collect_tuple())
            .ok_or("invalid input")?;
        network.valves.push(Valve {
            _id: network.valves.len(),
            name: name.to_owned(),
            flow_rate: flow_rate.parse()?,
            tunnel_names: pipes.split(", ").map(|x| x.to_owned()).collect_vec(),
            ..Default::default()
        });
    }

    for index in 0..network.valves.len() {
        let mut ids = network.valves[index]
            .tunnel_names
            .iter()
            .map(|name| network.valves.iter().position(|v2| &v2.name == name))
            .collect::<Option<Vec<_>>>()
            .ok_or("invalid input")?;
        network.valves[index].tunnels.append(&mut ids);
    }

    network.distance_map = create_distance_map(&network);
    network.start = network
        .valves
        .iter()
        .position(|x| x.name == "AA")
        .ok_or("start not found")?;

    Ok(network)
}

fn create_distance_map(network: &Network) -> Vec<Vec<Option<i32>>> {
    let mut distance_map = vec![vec![None; network.valves.len()]; network.valves.len()];
    for (from, to) in (0..network.valves.len())
        .combinations(2)
        .map(|x| (x[0], x[1]))
    {
        let mut visited = HashSet::from([from]);
        let mut queue = VecDeque::from([(from, 0)]);
        while let Some((current, distance)) = queue.pop_front() {
            if current == to {
                distance_map[from][to] = Some(distance);
                distance_map[to][from] = Some(distance);
                break;
            }

            for &next in &network.valves[current].tunnels {
                if visited.insert(next) {
                    queue.push_back((next, distance + 1))
                }
            }
        }
    }

    distance_map
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum AgentTask {
    Open(usize, i32),
    Ready(usize),
}
impl PartialOrd for AgentTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AgentTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (AgentTask::Open(v1, d1), AgentTask::Open(v2, d2)) => d1.cmp(d2).then(v1.cmp(v2)),
            (
                AgentTask::Open(v1, _) | AgentTask::Ready(v1),
                AgentTask::Open(v2, _) | AgentTask::Ready(v2),
            ) => v1.cmp(v2),
        }
    }
}

#[derive(Debug, Default)]
struct Network {
    valves: Vec<Valve>,
    start: usize,
    distance_map: Vec<Vec<Option<i32>>>,
}

#[derive(Debug, Default)]
struct Valve {
    _id: usize,
    name: String,
    flow_rate: i32,
    tunnels: Vec<usize>,
    tunnel_names: Vec<String>,
}
