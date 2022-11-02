use crate::config::Config;
use bracket_terminal::prelude::*;

/// Gets the mouse tile coordinates taking UI scaling into account.
pub fn get_mouse_tile_pos(config: &Config) -> PointF {
    let (x, y) = INPUT.lock().mouse_pixel_pos();
    PointF {
        x: x as f32 / config.tile_size_x as f32 / config.scale,
        y: y as f32 / config.tile_size_y as f32 / config.scale,
    }
}
