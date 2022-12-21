use crate::{helpers::re_capture_groups, solution::*, util::GenericResult};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day21;
impl Solution for Day21 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 21, "Monkey Math")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let monkeys = parse_input(ctx)?;
        let root = calc_monkey(&"root".to_owned(), &monkeys, &mut HashMap::new())?;
        Ok(root.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Err(NotImplementedError)?
    }
}

fn calc_monkey(
    name: &String,
    monkeys: &HashMap<String, Monkey>,
    cache: &mut HashMap<String, i64>,
) -> GenericResult<i64> {
    if let Some(value) = cache.get(name) {
        return Ok(*value);
    }

    let monkey = monkeys.get(name).ok_or("monkey not found")?;
    let result = match monkey {
        Monkey::Num(num) => *num,
        Monkey::Add(a, b) => calc_monkey(a, monkeys, cache)? + calc_monkey(b, monkeys, cache)?,
        Monkey::Sub(a, b) => calc_monkey(a, monkeys, cache)? - calc_monkey(b, monkeys, cache)?,
        Monkey::Mul(a, b) => calc_monkey(a, monkeys, cache)? * calc_monkey(b, monkeys, cache)?,
        Monkey::Div(a, b) => calc_monkey(a, monkeys, cache)? / calc_monkey(b, monkeys, cache)?,
    };
    cache.insert(name.to_owned(), result);

    Ok(result)
}

fn parse_input(ctx: &Context) -> GenericResult<HashMap<String, Monkey>> {
    let op_re = Regex::new(r"^(\S+): (\S+) (\S) (\S+)$").unwrap();
    let num_re = Regex::new(r"^(\S+): (-?\d+)$").unwrap();

    let mut monkeys = HashMap::new();
    for line in ctx.input().lines() {
        if let Some((name, a, op, b)) = re_capture_groups(&op_re, line)
            .and_then(|x| x.into_iter().map(|s| s.to_owned()).collect_tuple())
        {
            let monkey = match op.chars().next().ok_or("invalid input")? {
                '+' => Monkey::Add(a, b),
                '-' => Monkey::Sub(a, b),
                '*' => Monkey::Mul(a, b),
                '/' => Monkey::Div(a, b),
                _ => Err("invalid monkey operation")?,
            };
            monkeys.insert(name, monkey);
        } else if let Some((name, value_str)) = re_capture_groups(&num_re, line)
            .and_then(|x| x.into_iter().map(|s| s.to_owned()).collect_tuple())
        {
            monkeys.insert(name, Monkey::Num(value_str.parse()?));
        }
    }

    Ok(monkeys)
}

#[derive(Debug)]
enum Monkey {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}
