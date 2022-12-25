use crate::solution::*;
use derive_more::{Add, Constructor};
use std::{collections::HashMap, str::FromStr};

#[derive(Default)]
pub struct Day25;
impl Solution for Day25 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 25, "Full of Hot Air")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let numbers = ctx.input_values::<Snafu>()?;
        let sum = numbers
            .into_iter()
            .reduce(|a, x| a + x)
            .ok_or("empty input")?;

        Ok(sum.to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Ok("*".to_owned())
    }
}

#[derive(Constructor, Add)]
struct Snafu {
    dec_value: i32,
}
impl FromStr for Snafu {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = HashMap::from([('0', 0), ('1', 1), ('2', 2), ('-', -1), ('=', -2)]);
        let mut dec_value = 0;
        for (i, c) in s.chars().rev().enumerate() {
            dec_value += map.get(&c).ok_or("invalid snafu character")? * (5 as i32).pow(i as u32)
        }

        Ok(Snafu { dec_value })
    }
}
impl ToString for Snafu {
    fn to_string(&self) -> String {
        let map = HashMap::from([(0, '0'), (1, '1'), (2, '2'), (3, '='), (4, '-')]);
        let mut out = Vec::<char>::new();
        let mut remaining = self.dec_value;
        let mut power = 0;
        let mut carry = 0;

        for power in 1..10 {
            let v = remaining % (5 as i32).pow(power); // todo
            match v {
                _ => _ = *map.get(&(v as usize)).unwrap(),
                0 | 1 | 2 => carry = 0,
                3 | 4 => carry = 1,
                // _ => panic!(),
            };
            // out.push(c);
            // remaining -= v * (5 as i32).pow(power);
        }

        println!("{}", power);

        out.iter().rev().collect::<String>()
    }
}
