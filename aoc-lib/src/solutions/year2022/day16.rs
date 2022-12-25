use crate::{helpers::re_capture_groups, solution::*, util::GenericResult};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

#[derive(Default)]
pub struct Day16;
impl Solution for Day16 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 16, "Proboscidea Volcanium")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let network = parse_network(ctx)?;
        let routes = find_routes_for(30, &network);
        let (_, best_pressure) = routes
            .iter()
            .sorted_by(|(_, p1), (_, p2)| p2.cmp(&p1))
            .next()
            .ok_or("no path found")?;

        Ok(best_pressure.to_string())
    }

    // This solution is highly inefficient, runs for ~3 minutes on a i7-13700K
    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let network = parse_network(ctx)?;
        let routes = find_routes_for(26, &network);
        let best_pressure = find_best_route_pair_pressure(routes, ctx);

        Ok(best_pressure.to_string())
    }
}

fn find_best_route_pair_pressure(mut routes: Vec<(HashSet<usize>, i32)>, ctx: &Context) -> i32 {
    routes.sort_by(|(_, s1), (_, s2)| s2.cmp(&s1));
    let route_count = routes.len();
    let mut max = 0;
    for (a, b) in itertools::iproduct!(0..route_count, 0..route_count).filter(|(a, b)| a != b) {
        if a % 1000 == 0 && b == 0 {
            ctx.progress(a as f32 / route_count as f32);
        }

        let (s1, p1) = &routes[a];
        let (s2, p2) = &routes[b];
        let sum = *p1 + *p2;
        if sum <= max || s1.intersection(s2).next().is_some() {
            continue;
        }

        max = sum;
    }

    max
}

fn find_routes_for(time: i32, network: &Network) -> Vec<(HashSet<usize>, i32)> {
    let mut open_valves = vec![false; network.valves.len()];
    let mut routes = Vec::new();

    find_routes(
        &network,
        true,
        time,
        network.start,
        0,
        &mut open_valves,
        &mut vec![],
        &mut routes,
    );

    routes
}

fn find_routes(
    network: &Network,
    start: bool,
    time: i32,
    current: usize,
    mut pressure: i32,
    valves_open: &mut Vec<bool>,
    path: &mut Vec<usize>,
    routes: &mut Vec<(HashSet<usize>, i32)>,
) {
    let reachable = network.distance_map[current]
        .iter()
        .enumerate()
        .filter(|(i, d)| !valves_open[*i] && network.good_valves[*i] && d.is_some())
        .map(|(i, d)| (i, d.unwrap()))
        .collect_vec();

    // do not necessarily have to open the valve at the start, but it takes time if we do
    if !start {
        path.push(current);
        pressure += network.valves[current].flow_rate * time;
        routes.push((HashSet::from_iter(path.iter().map(|x| *x)), pressure));
    } else if network.good_valves[current] {
        valves_open[current] = true;
        find_routes(
            network,
            false,
            time - 1,
            current,
            pressure,
            valves_open,
            path,
            routes,
        );
        valves_open[current] = false;
    }

    for (next, distance) in reachable.iter() {
        let next_time = time - distance - 1;
        if next_time <= 1 {
            continue;
        }

        valves_open[*next] = true;
        find_routes(
            network,
            false,
            next_time,
            *next,
            pressure,
            valves_open,
            path,
            routes,
        );
        valves_open[*next] = false;
    }

    if !start {
        path.pop();
    }
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
    network.good_valves = network.valves.iter().map(|v| v.flow_rate > 0).collect_vec();

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

#[derive(Debug, Default)]
struct Network {
    valves: Vec<Valve>,
    good_valves: Vec<bool>,
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
