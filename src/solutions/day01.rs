use crate::api::solution::*;
use std::{thread, time::Duration};

#[derive(Default)]
pub struct Day01;
impl Solution for Day01 {
    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        if ctx.input.len() == 0 {
            return Err(format!("Length: {}", ctx.input.len()));
        }
        for p in 0..=50 {
            ctx.progress(p as f32 / 50.0)?;
            thread::sleep(Duration::from_millis(5));
        }
        Ok(ctx.input.len().to_string())
    }

    fn part2(&mut self, _ctx: &Context) -> SolutionResult {
        Ok("*".to_owned())
    }
}
