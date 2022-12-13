use crate::{solution::*, util::GenericResult};
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Default)]
pub struct Day13;
impl Solution for Day13 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 13, "Distress Signal")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let pairs = parse_packet_pairs(ctx)?;
        let mut sum = 0;
        for (index, (a, b)) in pairs.into_iter().enumerate() {
            if let Ordering::Less = a.cmp(&b) {
                sum += index + 1;
            }
        }

        Ok(sum.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let dividers = [
            Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
            Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
        ];

        let key = parse_packet_pairs(ctx)?
            .into_iter()
            .flat_map(|(a, b)| [a, b])
            .chain(dividers.clone())
            .sorted()
            .positions(|x| dividers.contains(&x))
            .fold(1, |a, x| a * (x + 1));

        Ok(key.to_string())
    }
}

fn parse_packet_pairs(ctx: &Context) -> GenericResult<Vec<(Packet, Packet)>> {
    let pairs = ctx
        .input()
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter_map(|x| parse_packet(&x.chars().collect_vec()).ok())
                .map(|x| x.0)
                .collect_tuple::<(_, _)>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or("invalid input format")?;

    Ok(pairs)
}

fn parse_packet(data: &[char]) -> GenericResult<(Packet, usize)> {
    let num_str = data
        .iter()
        .take_while(|c| if let '0'..='9' = c { true } else { false })
        .collect::<String>();
    if num_str.len() > 0 {
        return Ok((Packet::Int(num_str.parse()?), num_str.len()));
    }

    let mut index = 1; // skip our '['
    let mut items = Vec::new();
    while index < data.len() {
        match data[index] {
            '[' | '0'..='9' => {
                // process inner value
                let (value, size) = parse_packet(&data[index..])?;
                items.push(value);
                index += size;
            }
            ']' => break, // done processing our list
            ',' => index += 1,
            _ => Err("invalid input format")?,
        };
    }

    Ok((Packet::List(items), index + 1))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Int(i32),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                for index in 0..a.len().min(b.len()) {
                    let order = a[index].cmp(&b[index]);
                    match order {
                        Ordering::Equal => (),
                        _ => return order,
                    }
                }

                a.len().cmp(&b.len())
            }
            (Packet::Int(a), Packet::List(_)) => Packet::List(vec![Packet::Int(*a)]).cmp(other),
            (Packet::List(_), Packet::Int(b)) => self.cmp(&Packet::List(vec![Packet::Int(*b)])),
        }
    }
}
