use crate::config::Config;
use bracket_terminal::prelude::*;

/// Gets the mouse tile coordinates taking UI scaling into account.
pub fn get_mouse_tile_pos(config: &Config) -> PointF {
    let (x, y) = INPUT.lock().mouse_pixel_pos();
    PointF {
        x: x as f32 / config.tile_size_x as f32 / config.scale_x,
        y: y as f32 / config.tile_size_y as f32 / config.scale_y,
    }
}

pub fn distance2d_pythagoras_f32(start: &PointF, end: &PointF) -> f32 {
    let dx = (start.x.max(end.x) - start.x.min(end.x)) as f32;
    let dy = (start.y.max(end.y) - start.y.min(end.y)) as f32;
    ((dx * dx) + (dy * dy)).sqrt()
}
