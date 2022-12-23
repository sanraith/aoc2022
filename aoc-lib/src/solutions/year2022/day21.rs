use crate::{helpers::re_capture_groups, solution::*, util::GenericResult};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

static ROOT: &str = "root";
static HUMAN: &str = "humn";

#[derive(Default)]
pub struct Day21;
impl Solution for Day21 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 21, "Monkey Math")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let monkeys = parse_input(ctx)?;
        let root = calc_monkey(ROOT, &monkeys, &mut HashMap::new(), false)?;
        Ok(root.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let mut monkeys = parse_input(ctx)?;

        // replace root to Equ operation
        let root = monkeys.get(ROOT).ok_or("no root")?;
        let (expr_left, expr_right) = match root {
            Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => (a, b),
            _ => Err("invalid root")?,
        };
        monkeys.insert(
            ROOT.to_owned(),
            Monkey::Equ(expr_left.to_owned(), expr_right.to_owned()),
        );

        // Find variable and constant side of root
        let root = monkeys.get(ROOT).ok_or("no root")?;
        let (expr_left, expr_right) = match root {
            Monkey::Equ(a, b) => (a, b),
            _ => Err("invalid root")?,
        };
        let left = calc_monkey(expr_left, &monkeys, &mut HashMap::new(), true);
        let right = calc_monkey(expr_right, &monkeys, &mut HashMap::new(), true);
        let (mut left, mut right) = match (left, right) {
            (Err(_), Ok(constant)) => (expr_left, constant),
            (Ok(constant), Err(_)) => (expr_right, constant),
            _ => Err("invalid result")?,
        };

        // Solve for humn
        while left != HUMAN {
            let current = monkeys.get(left).ok_or("invalid input")?;
            let (a_ref, b_ref) = match current {
                Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => {
                    (a, b)
                }
                _ => Err("invalid input")?,
            };

            let a_result = calc_monkey(a_ref, &monkeys, &mut HashMap::new(), true);
            let b_result = calc_monkey(b_ref, &monkeys, &mut HashMap::new(), true);
            let (a, b) = match (&a_result, &b_result) {
                (Ok(c), Err(_)) => (c, b_ref),
                (Err(_), Ok(c)) => (c, a_ref),
                _ => Err("invalid input")?,
            };

            match current {
                Monkey::Add(_, _) => right -= a,
                Monkey::Sub(_, _) if a_result.is_err() => right += a,
                Monkey::Sub(_, _) => right = a - right,
                Monkey::Mul(_, _) => right /= a,
                Monkey::Div(_, _) if a_result.is_err() => right *= a,
                Monkey::Div(_, _) => right = a / right,
                _ => Err("invalid input")?,
            }
            left = b;
        }

        Ok(right.to_string())
    }
}

fn calc_monkey(
    name: &str,
    monkeys: &HashMap<String, Monkey>,
    cache: &mut HashMap<String, i64>,
    skip_human: bool,
) -> GenericResult<i64> {
    if skip_human && name == HUMAN {
        Err("human")?;
    }

    if let Some(value) = cache.get(name) {
        return Ok(*value);
    }

    let monkey = monkeys.get(name).ok_or("monkey not found")?;
    let result = match monkey {
        Monkey::Num(num) => *num,
        Monkey::Add(a, b) => {
            calc_monkey(a, monkeys, cache, skip_human)?
                + calc_monkey(b, monkeys, cache, skip_human)?
        }
        Monkey::Sub(a, b) => {
            calc_monkey(a, monkeys, cache, skip_human)?
                - calc_monkey(b, monkeys, cache, skip_human)?
        }
        Monkey::Mul(a, b) => {
            calc_monkey(a, monkeys, cache, skip_human)?
                * calc_monkey(b, monkeys, cache, skip_human)?
        }
        Monkey::Div(a, b) => {
            calc_monkey(a, monkeys, cache, skip_human)?
                / calc_monkey(b, monkeys, cache, skip_human)?
        }
        _ => Err("invalid monkey")?,
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
    Equ(String, String),
}
