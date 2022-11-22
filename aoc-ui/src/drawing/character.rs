use super::drawing_base::{Drawable, DrawingBase};
use bracket_terminal::prelude::*;

#[derive(Clone)]
pub struct Character {
    pub glyph: char,
    pub base: DrawingBase,
}
impl Drawable for Character {
    fn draw(&self, _ctx: &BTerm, batch: &mut DrawBatch) {
        batch.set_fancy(
            PointF {
                x: self.base.pos.x,
                y: self.base.pos.y + 1.0,
            },
            self.base.z_order,
            Degrees::new(self.base.rotation),
            PointF::new(self.base.scale, self.base.scale),
            ColorPair::new(
                RGBA::from_u8(255, 255, 255, (self.base.opaqueness * 255.0) as u8),
                RGBA::from_u8(0, 0, 0, 0),
            ),
            to_cp437(self.glyph),
        );
    }

    fn base(&self) -> &DrawingBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut DrawingBase {
        &mut self.base
    }
}
