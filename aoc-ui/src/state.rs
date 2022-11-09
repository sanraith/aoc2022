use crate::{config::Config, entry, snowflake::SnowflakeManager, util::get_mouse_tile_pos};
use bracket_terminal::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub struct UiState {
    config: Rc<RefCell<Config>>,
    total_time: f32,
    snowflake_manager: SnowflakeManager,
}
impl GameState for UiState {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.total_time += ctx.frame_time_ms;

        // Exit on Esc key
        if let Some(key) = ctx.key {
            match key {
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

        // Apply config changes from javascript
        let outside_config = entry::JS_CONFIG.lock().unwrap();
        if outside_config.scale > 0.0 {
            normal_batch.print(
                Point::from_tuple((1, 5)),
                format!("Scale: {:.3}", outside_config.scale),
            );
            (*self.config.borrow_mut()).scale = outside_config.scale as f32;
        }

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
        let config = Rc::from(RefCell::new(config));
        UiState {
            config: Rc::clone(&config),
            snowflake_manager: SnowflakeManager::new(Rc::clone(&config)),
            total_time: 0.0,
        }
    }

    fn handle_status(&self, ctx: &BTerm, batch: &mut DrawBatch) {
        batch.print(Point::from_tuple((1, 2)), "Advent of Code 2022");
        let status = format!("FPS: {:>2}", ctx.fps as i32);
        batch.print(Point::from_tuple((1, 4)), status);
    }

    fn handle_mouse(&self, batch: &mut DrawBatch) {
        let mp = get_mouse_tile_pos(&self.config.borrow());
        batch.set_fancy(
            PointF {
                x: mp.x - 0.4,
                y: mp.y,
            },
            0,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            ColorPair::new(WHITE, RGBA::from_u8(0, 0, 0, 0)),
            to_cp437('.'),
        );
    }
}
