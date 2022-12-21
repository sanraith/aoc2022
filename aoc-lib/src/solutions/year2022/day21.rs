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
        let root = calc_monkey("root", &monkeys, &mut HashMap::new())?;
        Ok(root.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let mut monkeys = parse_input(ctx)?;
        let root = monkeys.get("root").ok_or("no root")?;
        let (expr_left, expr_right) = match root {
            Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => (a, b),
            _ => Err("invalid root")?,
        };
        monkeys.insert(
            "root".to_owned(),
            Monkey::Equ(expr_left.to_owned(), expr_right.to_owned()),
        );
        let s = print_eq_simplified("root", &monkeys, &mut HashMap::new());
        println!("{:?}", s);

        // let mut c = Computer::<f64>::default();
        // println!("{}", c.eval(&s).unwrap());

        let root = monkeys.get("root").ok_or("no root")?;
        let (expr_left, expr_right) = match root {
            Monkey::Equ(a, b) => (a, b),
            _ => Err("invalid root")?,
        };
        let left = calc_monkey_2(expr_left, &monkeys, &mut HashMap::new());
        let right = calc_monkey_2(expr_right, &monkeys, &mut HashMap::new());
        let (mut left, mut right) = match (left, right) {
            (Err(_), Ok(constant)) => (expr_left, constant),
            (Ok(constant), Err(_)) => (expr_right, constant),
            _ => Err("invalid result")?,
        };
        println!("{} = {}", left, right);

        while left != "humn" {
            let current = monkeys.get(left).ok_or("invalid input")?;
            let (a_ref, b_ref) = match current {
                Monkey::Add(a, b) | Monkey::Sub(a, b) | Monkey::Mul(a, b) | Monkey::Div(a, b) => {
                    (a, b)
                }
                Monkey::Num(a) => panic!("num"),
                Monkey::Equ(_, _) => panic!("equ"),
            };
            let (a_res, b_res) = (
                calc_monkey_2(a_ref, &monkeys, &mut HashMap::new()),
                calc_monkey_2(b_ref, &monkeys, &mut HashMap::new()),
            );
            let (a, b) = match (&a_res, &b_res) {
                (Ok(c), Err(_)) => (c, b_ref),
                (Err(_), Ok(c)) => (c, a_ref),
                (Ok(_), Ok(_)) => panic!("ok"),
                (Err(_), Err(_)) => panic!("err"),
            };

            match current {
                Monkey::Add(_, _) => right -= a,
                Monkey::Sub(a1, b1) if a_res.is_err() => right += a,
                Monkey::Sub(a1, b1) => right = a - right,
                Monkey::Mul(_, _) => right /= a,
                Monkey::Div(_, _) if a_res.is_err() => right *= a,
                Monkey::Div(_, _) => right = a / right,
                Monkey::Num(_) => Err("should not be num")?,
                Monkey::Equ(_, _) => Err("should not be equ")?,
            }
            left = b;
        }

        Ok(right.to_string())
    }
}

#[derive(Clone, Debug)]
enum Asd {
    String(String),
    Num(i64),
}

fn calc_monkey_2(
    name: &str,
    monkeys: &HashMap<String, Monkey>,
    cache: &mut HashMap<String, i64>,
) -> GenericResult<i64> {
    if name == "humn" {
        Err("human")?;
    }

    if let Some(value) = cache.get(name) {
        return Ok(*value);
    }

    let monkey = monkeys.get(name).ok_or("monkey not found")?;
    let result = match monkey {
        Monkey::Num(num) => *num,
        Monkey::Add(a, b) => calc_monkey_2(a, monkeys, cache)?
            .checked_add(calc_monkey_2(b, monkeys, cache)?)
            .ok_or("add")?,
        Monkey::Sub(a, b) => calc_monkey_2(a, monkeys, cache)?
            .checked_sub(calc_monkey_2(b, monkeys, cache)?)
            .ok_or("sub")?,
        Monkey::Mul(a, b) => calc_monkey_2(a, monkeys, cache)? * calc_monkey_2(b, monkeys, cache)?,
        Monkey::Div(a, b) => calc_monkey_2(a, monkeys, cache)? / calc_monkey_2(b, monkeys, cache)?,
        _ => Err("invalid monkey")?,
    };
    cache.insert(name.to_owned(), result);

    Ok(result)
}

fn print_eq_simplified(
    name: &str,
    monkeys: &HashMap<String, Monkey>,
    cache: &mut HashMap<String, Asd>,
) -> Asd {
    if let Some(value) = cache.get(name) {
        return value.clone();
    }

    if name == "humn" {
        return Asd::String("x".to_owned());
    }

    let monkey = monkeys.get(name).unwrap();
    let parts = match monkey {
        Monkey::Num(num) => (Asd::Num(*num), Asd::Num(0)),
        Monkey::Add(a, b)
        | Monkey::Sub(a, b)
        | Monkey::Mul(a, b)
        | Monkey::Div(a, b)
        | Monkey::Equ(a, b) => (
            print_eq_simplified(a, monkeys, cache),
            print_eq_simplified(b, monkeys, cache),
        ),
    };

    let result = match parts {
        (Asd::Num(a), Asd::Num(b)) => match monkey {
            Monkey::Num(_) => Some(Asd::Num(a)),
            Monkey::Add(_, _) => Some(Asd::Num(a + b)),
            Monkey::Sub(_, _) => Some(Asd::Num(a - b)),
            Monkey::Mul(_, _) => Some(Asd::Num(a * b)),
            Monkey::Div(_, _) => Some(Asd::Num(a / b)),
            Monkey::Equ(_, _) => panic!(),
        },
        _ => None,
    };

    if let None = result {
        let result = [parts.0, parts.1]
            .iter()
            .map(|x| match x {
                Asd::String(x) => x.to_owned(),
                Asd::Num(x) => x.to_string(),
            })
            .collect_vec();
        let (a, b) = (result[0].clone(), result[1].clone());

        return Asd::String(match monkey {
            Monkey::Add(_, _) => format!("({} + {})", a, b),
            Monkey::Sub(_, _) => format!("({} - {})", a, b),
            Monkey::Mul(_, _) => format!("{} * {}", a, b),
            Monkey::Div(_, _) => format!("{} / {}", a, b),
            Monkey::Equ(_, _) => format!("{} = {}", a, b),
            _ => panic!("{:?}", monkey),
        });
    } else {
        return result.unwrap();
    }
}

fn print_eq(
    name: &str,
    monkeys: &HashMap<String, Monkey>,
    cache: &mut HashMap<String, String>,
) -> String {
    if let Some(value) = cache.get(name) {
        return value.to_owned();
    }

    if name == "humn" {
        return "x".to_owned();
    }

    let monkey = monkeys.get(name).unwrap();
    let result = match monkey {
        Monkey::Num(x) => x.to_string(),
        Monkey::Add(a, b) => format!(
            "({} + {})",
            print_eq(a, monkeys, cache),
            print_eq(b, monkeys, cache)
        ),
        Monkey::Sub(a, b) => format!(
            "({} - {})",
            print_eq(a, monkeys, cache),
            print_eq(b, monkeys, cache)
        ),
        Monkey::Mul(a, b) => format!(
            "{} * {}",
            print_eq(a, monkeys, cache),
            print_eq(b, monkeys, cache)
        ),
        Monkey::Div(a, b) => format!(
            "{} / {}",
            print_eq(a, monkeys, cache),
            print_eq(b, monkeys, cache)
        ),
        Monkey::Equ(a, b) => format!(
            "{} = {}",
            print_eq(a, monkeys, cache),
            print_eq(b, monkeys, cache)
        ),
    };

    result
}

fn calc_monkey(
    name: &str,
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
