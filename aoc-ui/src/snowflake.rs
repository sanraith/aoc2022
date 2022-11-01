use bracket_terminal::prelude::*;

pub struct Snowflake {
    pub base: Point,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub max_y: f32,
    pub elapsed: f32,
    pub done: bool,
}
impl Default for Snowflake {
    fn default() -> Self {
        Self {
            base: Point { x: 0, y: 0 },
            x: Default::default(),
            y: Default::default(),
            vx: Default::default(),
            vy: Default::default(),
            max_y: Default::default(),
            elapsed: Default::default(),
            done: Default::default(),
        }
    }
}
impl Snowflake {
    pub fn progress(&mut self, ctx: &BTerm) {
        self.elapsed += ctx.frame_time_ms;
        self.y = self.base.y as f32 + self.elapsed * self.vy / 1000.0;
        self.x = self.base.x as f32 + (self.elapsed / 300.0).sin() * self.vx;
        if self.y >= self.max_y {
            self.done = true;
        }
    }
    pub fn draw(&self, batch: &mut DrawBatch) {
        batch.print(Point::from_tuple((self.x as i32, self.y as i32)), ".");
    }
}
