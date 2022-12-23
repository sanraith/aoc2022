use super::drawing_base::{Drawable, DrawingBase};
use bracket_terminal::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Default, Clone, Debug)]
pub struct Snowflake {
    pub base: DrawingBase,
}
impl Drawable for Snowflake {
    fn draw(&self, _ctx: &BTerm, batch: &mut DrawBatch) {
        let c = self.base.color;
        let color: (u8, u8, u8, u8) = (c.0, c.1, c.2, (self.opaqueness * 255.0) as u8);

        batch.set_fancy(
            self.pos.clone(),
            self.z_order,
            Degrees::new(self.rotation),
            PointF::new(self.scale * 0.5, self.scale * 1.0),
            ColorPair::new(color, RGBA::from_u8(0, 0, 0, 0)),
            to_cp437('*'),
        );
    }

    fn base(&self) -> &DrawingBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut DrawingBase {
        &mut self.base
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
