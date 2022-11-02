use crate::{config::Config, snowflake::SnowflakeManager};
use bracket_terminal::prelude::*;

pub struct UiState {
    config: Config,
    total_time: f32,
    snowflake_manager: SnowflakeManager,
}
impl GameState for UiState {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.total_time += ctx.frame_time_ms;

        // Exit on Q or Esc key
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Q => ctx.quit(),
                VirtualKeyCode::Escape => ctx.quit(),
                _ => (),
            }
        }

        let mut normal_batch = DrawBatch::new();
        normal_batch.target(0);
        normal_batch.cls();

        let mut fancy_batch = DrawBatch::new();
        fancy_batch.target(1);
        fancy_batch.cls();

        self.snowflake_manager.tick(ctx, &mut fancy_batch);
        self.handle_status(ctx, &mut normal_batch);
        self.handle_mouse(&mut fancy_batch);

        fancy_batch.submit(2).expect("Render error");
        normal_batch.submit(1).expect("Render error");
        render_draw_buffer(ctx).expect("Render error");
    }
}
impl UiState {
    pub fn new(config: Config) -> Self {
        UiState {
            config,
            total_time: 0.0,
            snowflake_manager: SnowflakeManager::new(config),
        }
    }

    fn handle_status(&self, ctx: &mut BTerm, batch: &mut DrawBatch) {
        let status = format!(
            "{:>3} FPS, runtime: {}s",
            ctx.fps as i32,
            (self.total_time / 1000.0) as i32
        );
        batch.print(Point::from_tuple((1, 2)), status);
    }

    fn handle_mouse(&self, batch: &mut DrawBatch) {
        let (x, y) = INPUT.lock().mouse_pixel_pos();
        batch.set_fancy(
            PointF {
                x: x as f32 / self.config.tile_size_x as f32 - 0.4,
                y: y as f32 / self.config.tile_size_y as f32 + 0.2,
            },
            0,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            ColorPair::new(WHITE, RGBA::from_u8(0, 0, 0, 0)),
            to_cp437('.'),
        );
    }
}
