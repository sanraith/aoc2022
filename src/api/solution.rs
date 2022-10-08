pub type SolutionResult = Result<String, String>;

pub struct Context<'a> {
    pub input: &'a str,
}

pub trait Solution
where
    Self: Default,
{
    fn new() -> Self {
        Self::default()
    }
    fn part1(&self, ctx: &Context) -> SolutionResult;
    fn part2(&self, ctx: &Context) -> SolutionResult;
}
