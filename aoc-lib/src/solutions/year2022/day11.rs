use crate::{solution::*, util::GenericResult};
use itertools::Itertools;
use regex::Regex;

#[derive(Default)]
pub struct Day11;
impl Solution for Day11 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 11, "Monkey in the Middle")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        const DIVIDE_WORRY: i64 = 3;
        const ROUNDS: i64 = 20;
        let monkey_business_level = calc_monkey_business(ctx, DIVIDE_WORRY, ROUNDS)?;
        Ok(monkey_business_level.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        const DIVIDE_WORRY: i64 = 1;
        const ROUNDS: i64 = 10000;
        let monkey_business_level = calc_monkey_business(ctx, DIVIDE_WORRY, ROUNDS)?;
        Ok(monkey_business_level.to_string())
    }
}

fn calc_monkey_business(ctx: &Context, divide_worry: i64, rounds: i64) -> GenericResult<i64> {
    let mut monkeys = parse_monkeys(ctx)?;
    let modulus = monkeys.iter().fold(1, |a, m| a * m.test_num);
    for _round in 0..rounds {
        for from_idx in 0..monkeys.len() {
            while let Some((item, to_idx)) = monkeys[from_idx].throw_item(divide_worry, modulus) {
                monkeys[to_idx].catch_item(item);
            }
        }
    }

    let monkey_business_level = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .fold(1, |a, x| a * x);

    Ok(monkey_business_level)
}

fn parse_monkeys(ctx: &Context) -> GenericResult<Vec<Monkey>> {
    let err = "invalid input";
    let monkey_re = Regex::new(r"items: (.*)\n.*= (\S+) (\S) (\S+)\n.* (\d+)\n.* (\d+)\n.* (\d+)")?;

    let input = ctx.input();
    let definitions = input.split("\n\n");
    let mut monkeys = Vec::new();
    for monkey_def in definitions {
        let (_, items, a, op, b, test_num, true_target, false_target) = monkey_re
            .captures(monkey_def)
            .ok_or(err)?
            .iter()
            .filter_map(|m| m)
            .map(|m| m.as_str().to_owned())
            .collect_tuple()
            .ok_or(err)?;
        let items = items
            .split(", ")
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let test_num = test_num.parse()?;
        let true_target = true_target.parse()?;
        let false_target = false_target.parse()?;

        let operation = move |old: i64| {
            let parse_num = |x: &String| match x.as_str() {
                "old" => old,
                num_str => num_str.parse().unwrap(),
            };
            let a = parse_num(&a);
            let b = parse_num(&b);

            match op.as_str() {
                "+" => a + b,
                _ => a * b,
            }
        };

        monkeys.push(Monkey {
            items,
            inspected_count: 0,
            operation: Box::new(operation),
            test_num,
            true_target,
            false_target,
        });
    }

    Ok(monkeys)
}

struct Monkey {
    items: Vec<i64>,
    inspected_count: i64,
    operation: Box<dyn Fn(i64) -> i64>,
    test_num: i64,
    true_target: usize,
    false_target: usize,
}
impl Monkey {
    fn throw_item(&mut self, divide_worry: i64, modulus: i64) -> Option<(i64, usize)> {
        if self.items.len() == 0 {
            return None;
        }

        self.inspected_count += 1;
        let worry = (self.operation)(self.items.remove(0)) % modulus / divide_worry;

        match worry % self.test_num {
            0 => Some((worry, self.true_target)),
            _ => Some((worry, self.false_target)),
        }
    }

    fn catch_item(&mut self, item: i64) {
        self.items.push(item);
    }
}
