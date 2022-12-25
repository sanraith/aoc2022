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
        let snafu_numbers = ctx.input_values::<Snafu>()?;
        let sum = snafu_numbers
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
    dec_value: i64,
}
impl FromStr for Snafu {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = HashMap::from([('0', 0), ('1', 1), ('2', 2), ('-', -1), ('=', -2)]);
        let mut dec_value = 0;
        for (i, c) in s.chars().rev().enumerate() {
            dec_value += map.get(&c).ok_or("invalid snafu character")? * (5 as i64).pow(i as u32)
        }

        Ok(Snafu { dec_value })
    }
}
impl ToString for Snafu {
    fn to_string(&self) -> String {
        let map = HashMap::from([(0, '0'), (1, '1'), (2, '2'), (3, '='), (4, '-')]);
        let mut out = Vec::new();
        let mut remaining = self.dec_value;

        while remaining > 0 {
            let digit_value = remaining % 5;
            match map.get(&digit_value) {
                Some(digit) => out.push(*digit),
                None => out.push('?'),
            };

            let carry = match digit_value {
                3 | 4 => 1,
                _ => 0,
            };
            remaining = remaining / 5 + carry;
        }

        out.iter().rev().collect::<String>()
    }
}
