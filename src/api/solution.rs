pub type StringResult = Result<String, String>;

pub struct Context<'a> {
    pub input: &'a str,
    pub progress: fn(f32) -> (),
}
impl<'a> Default for Context<'a> {
    fn default() -> Self {
        Self {
            input: &"",
            progress: |_| (),
        }
    }
}
impl<'a> Context<'a> {
    /// Updates the current progress percentage.
    /// value range: 0..1
    pub fn progress(&self, value: f32) -> Result<(), String> {
        if value < 0.0 || value > 1.0 {
            return Err(format!("Invalid progress value: {}", value));
        }
        Ok((self.progress)(value))
    }
}

pub trait Solution
where
    Self: Default,
{
    fn new() -> Self {
        Self::default()
    }
    fn part1(&self, ctx: &Context) -> StringResult;
    fn part2(&self, ctx: &Context) -> StringResult;
}
