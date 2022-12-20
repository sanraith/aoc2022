use crate::{solution::*, util::GenericResult};
use itertools::Itertools;

#[derive(Default)]
pub struct Day20;
impl Solution for Day20 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 20, "Grove Positioning System")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let (numbers, mut ring) = parse_input(ctx, 1)?;
        mix(&numbers, &mut ring);
        let sum = grove_coordinate_sum(&numbers, &ring)?;

        Ok(sum.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let (numbers, mut ring) = parse_input(ctx, 811589153)?;
        for iteration in 0..10 {
            ctx.progress(iteration as f32 / 10.0);
            mix(&numbers, &mut ring);
        }
        let sum = grove_coordinate_sum(&numbers, &ring)?;

        Ok(sum.to_string())
    }
}

fn mix(numbers: &Vec<i64>, ring: &mut Vec<RingItem>) {
    for (index, &number) in numbers.iter().enumerate() {
        let steps = match number {
            _ if number < 0 => (number - 1) % (numbers.len() - 1) as i64,
            _ => number % (numbers.len() - 1) as i64,
        };
        if steps == 0 {
            continue;
        }

        // cut from ring
        let prev = ring[index].prev_idx;
        let next = ring[index].next_idx;
        ring[prev].next_idx = next;
        ring[next].prev_idx = prev;

        // insert to ring
        let target_prev = ring[index].step(steps, &ring);
        let target_next = ring[target_prev].next_idx;
        ring[target_prev].next_idx = index;
        ring[target_next].prev_idx = index;
        ring[index].prev_idx = target_prev;
        ring[index].next_idx = target_next;
    }
}

fn grove_coordinate_sum(numbers: &Vec<i64>, ring: &Vec<RingItem>) -> GenericResult<i64> {
    let zero_idx = numbers.iter().position(|&x| x == 0).ok_or("no 0 found")?;
    let item = &ring[zero_idx];
    let sum = [1000, 2000, 3000]
        .into_iter()
        .map(|count| ring[item.step(count, &ring)].value)
        .sum::<i64>();

    Ok(sum)
}

fn parse_input(ctx: &Context, key: i64) -> GenericResult<(Vec<i64>, Vec<RingItem>)> {
    let numbers = ctx
        .input_values::<i64>()?
        .into_iter()
        .map(|x| x * key)
        .collect_vec();
    let count = numbers.len();
    let ring = numbers
        .iter()
        .enumerate()
        .map(|(index, &value)| RingItem {
            index,
            value,
            prev_idx: if index == 0 { count - 1 } else { index - 1 },
            next_idx: if index >= count - 1 { 0 } else { index + 1 },
        })
        .collect_vec();

    Ok((numbers, ring))
}

#[derive(Debug, Clone)]
struct RingItem {
    index: usize,
    value: i64,
    prev_idx: usize,
    next_idx: usize,
}
impl RingItem {
    fn step(&self, mut count: i64, ring: &Vec<RingItem>) -> usize {
        let direction = count.signum();
        let mut current_idx = self.index;
        while count != 0 {
            match direction {
                -1 => current_idx = ring.get(current_idx).unwrap().prev_idx,
                _ => current_idx = ring.get(current_idx).unwrap().next_idx,
            };
            count -= direction;
        }

        current_idx
    }
}

#[allow(dead_code)]
fn print_ring(ring: &Vec<RingItem>) {
    let mut current = 0;
    loop {
        print!("{}, ", ring[current].value);
        current = ring[current].step(1, ring);
        if current == 0 {
            break;
        }
    }
    println!();
}
