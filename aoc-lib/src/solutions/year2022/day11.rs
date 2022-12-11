use crate::{solution::*, util::GenericResult};
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;

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
    let monkeys = parse_monkeys(ctx)?;
    let modulus = monkeys.iter().fold(1, |a, m| a * m.test_num);
    for _ in 0..rounds {
        for monkey in &monkeys {
            while monkey.has_items() {
                let (item, target) = monkey.throw_item(divide_worry, modulus);
                monkeys[target].catch_item(item);
            }
        }
    }

    let monkey_business_level = monkeys
        .iter()
        .map(|m| m.inspected_count())
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

        let operation = move |old: i64, modulus: i64| {
            let parse_num = |x: &String| match x.as_str() {
                "old" => old,
                num_str => num_str.parse().unwrap(),
            };
            let a = parse_num(&a);
            let b = parse_num(&b);

            match op.as_str() {
                "+" => (a + b) % modulus,
                _ => (a * b) % modulus,
            }
        };

        monkeys.push(Monkey {
            items: RefCell::new(items),
            inspected_count: RefCell::new(0),
            operation: Box::new(operation),
            test_num,
            true_target,
            false_target,
        });
    }

    Ok(monkeys)
}

struct Monkey {
    items: RefCell<Vec<i64>>,
    inspected_count: RefCell<i64>,
    operation: Box<dyn Fn(i64, i64) -> i64>,
    test_num: i64,
    true_target: usize,
    false_target: usize,
}
impl Monkey {
    fn throw_item(&self, divide_worry: i64, modulus: i64) -> (i64, usize) {
        *self.inspected_count.borrow_mut() += 1;
        let mut worry = self.items.borrow_mut().remove(0);
        worry = (self.operation)(worry, modulus) / divide_worry;
        if worry % self.test_num == 0 {
            (worry, self.true_target)
        } else {
            (worry, self.false_target)
        }
    }

    fn catch_item(&self, item: i64) {
        self.items.borrow_mut().push(item);
    }

    fn has_items(&self) -> bool {
        self.items.borrow().len() > 0
    }

    fn inspected_count(&self) -> i64 {
        *self.inspected_count.borrow()
    }
}
