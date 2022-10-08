use crate::api::solution::*;

#[derive(Default)]
pub struct Day01;
impl Solution for Day01 {
    fn part1(&self, ctx: &Context) -> SolutionResult {
        if ctx.input.len() == 0 {
            return Err(format!("Length: {}", ctx.input.len()));
        }
        Ok(ctx.input.len().to_string())
    }

    fn part2(&self, _ctx: &Context) -> SolutionResult {
        Ok("*".to_owned())
    }
}
