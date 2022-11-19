use bracket_terminal::prelude::{BTerm, DrawBatch, PointF};

#[derive(Clone, Debug)]
pub struct DrawingBase {
    pub pos: PointF,
    pub scale: f32,
    /** Rotation in degrees. */
    pub rotation: f32,
    /** Transparency ranging 0.0..=1.0. */
    pub transparency: f32,
    pub visible: bool,
}
impl Default for DrawingBase {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            scale: 1.0,
            rotation: 0.0,
            transparency: 1.0,
            visible: true,
        }
    }
}

pub trait Drawable {
    fn draw(&self, ctx: &BTerm, batch: &mut DrawBatch);
    fn base(&self) -> &DrawingBase;
    fn base_mut(&mut self) -> &mut DrawingBase;
}
