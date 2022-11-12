use super::drawing_base::{Drawable, DrawingBase};
use bracket_terminal::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct Snowflake {
    base: DrawingBase,
}
impl Drawable for Snowflake {
    fn draw(&self, _ctx: &BTerm, batch: &mut DrawBatch) {
        let color: (u8, u8, u8, u8) = (200, 200, 200, (self.transparency * 255.0) as u8);

        batch.set_fancy(
            self.pos.clone(),
            0,
            Degrees::new(self.rotation),
            PointF::new(self.scale * 0.5, self.scale * 1.0),
            ColorPair::new(color, RGBA::from_u8(0, 0, 0, 0)),
            to_cp437('*'),
        );
    }
}

impl Deref for Snowflake {
    type Target = DrawingBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for Snowflake {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
