use bracket_terminal::prelude::{BTerm, DrawBatch, PointF};

#[derive(Clone, Debug)]
pub struct DrawingBase {
    pub pos: PointF,
    pub scale: f32,
    pub color: (u8, u8, u8, u8),
    /** Rotation in degrees. */
    pub rotation: f32,
    /** Transparency ranging 0.0..=1.0. */
    pub opaqueness: f32,
    pub visible: bool,
    pub z_order: i32,
}
impl Default for DrawingBase {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            scale: 1.0,
            color: (255, 255, 255, 255),
            rotation: 0.0,
            opaqueness: 1.0,
            visible: true,
            z_order: 0,
        }
    }
}

pub trait Drawable {
    fn draw(&self, ctx: &BTerm, batch: &mut DrawBatch);
    fn base(&self) -> &DrawingBase;
    fn base_mut(&mut self) -> &mut DrawingBase;
}
