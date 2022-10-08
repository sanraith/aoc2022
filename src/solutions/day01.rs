use crate::api::solution::{Context, Solution, StringResult};

#[derive(Default)]
pub struct Day01;
impl Solution for Day01 {
    fn part1(&self, ctx: &Context) -> StringResult {
        if ctx.input.len() == 0 {
            return Err(format!("Length: {}", ctx.input.len()));
        }
        Ok(ctx.input.to_owned())
    }

    fn part2(&self, _ctx: &Context) -> StringResult {
        Ok("*".to_owned())
    }
}
