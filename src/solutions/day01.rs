use crate::api::solution::*;

#[derive(Default)]
pub struct Day01;
impl Solution for Day01 {
    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let items = ctx.input_items::<i32>()?;
        let count =
            items
                .iter()
                .enumerate()
                .skip(1)
                .fold(0, |a, (i, x)| if &items[i - 1] < x { a + 1 } else { a });

        Ok(count.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let count = ctx
            .input_items::<i32>()?
            .windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|&x| x[0] < x[1])
            .count();

        Ok(count.to_string())
    }
}
