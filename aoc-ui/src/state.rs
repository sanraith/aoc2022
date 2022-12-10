use crate::{
    char_image,
    config::Config,
    js_interop::{self, JS_BRIDGE},
    manager::{flake_text_manager::FlakeCharLine, snowflake_manager::SnowflakeManager},
    util::get_mouse_tile_pos,
    wasm_runner::WasmRunner,
};
use aoc::{
    core::solution_runner::{LocalSyncStream, SolutionRunner, SyncStream, ThreadSolutionRunner},
    solutions,
};
use bracket_terminal::prelude::*;
use rand::Rng;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct UiState {
    config: Rc<RefCell<Config>>,
    total_time: f32,
    snowflake_manager: SnowflakeManager,
    text_manager: FlakeCharLine,
    solve_state: Option<Arc<Mutex<LocalSyncStream>>>,
}
impl GameState for UiState {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.total_time += ctx.frame_time_ms;

        // POC solution runner
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::F5 => {
                    let runner: Box<dyn SolutionRunner<LocalSyncStream>> =
                        match JS_BRIDGE.lock().unwrap().worker_wrapper {
                            Some(_) => Box::new(WasmRunner {}),
                            None => Box::new(ThreadSolutionRunner {}),
                        };
                    self.solve_state = Some(runner.run(
                        solutions::create_list().last().unwrap().info.year_day(),
                        aoc::core::solution_runner::Input::Default,
                    ));
                }
                _ => (),
            }
        }

        let mut normal_batch = DrawBatch::new();
        normal_batch.target(0);
        normal_batch.cls();

        let mut fancy_batch = DrawBatch::new();
        fancy_batch.target(1);
        fancy_batch.cls();

        self.handle_js_scale_changes(&mut normal_batch);
        self.handle_bracket_events(ctx);
        self.handle_mouse(&mut fancy_batch);

        self.handle_solution_progress_updates();
        self.handle_flake_text_manager(ctx, &mut fancy_batch);
        self.text_manager.tick(ctx, &mut fancy_batch);
        self.print_status(ctx, &mut normal_batch);

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
            text_manager: FlakeCharLine::new(PointF::from((1.0, 8.0))),
            total_time: 0.0,
            solve_state: None,
        }
    }

    fn print_status(&self, ctx: &BTerm, batch: &mut DrawBatch) {
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

    fn handle_bracket_events(&mut self, ctx: &mut BTerm) {
        INPUT.lock().for_each_message(|event| {
            match event {
                // Window events
                BEvent::Resized { new_size, .. } => self.handle_window_resize(&event, new_size),
                BEvent::CloseRequested => ctx.quit(),

                // Keyboard events
                BEvent::KeyboardInput {
                    key: VirtualKeyCode::Back,
                    pressed: true,
                    ..
                } => {
                    self.text_manager.text.pop();
                }
                BEvent::KeyboardInput {
                    key: VirtualKeyCode::Return,
                    pressed: true,
                    ..
                } => self.text_manager.text.clear(),
                BEvent::Character { c } if char_image::CHARACTER_IMAGES.contains_key(&c) => {
                    self.text_manager.add_char(c)
                }
                _ => (),
            };
        });

        // Handle keyboard events from JS
        let js_unhandled_keys = JS_BRIDGE
            .lock()
            .unwrap()
            .unhandled_keys
            .drain(..)
            .collect::<Vec<_>>();
        for key in js_unhandled_keys {
            match key.as_str() {
                "Backspace" => {
                    self.text_manager.text.pop();
                }
                "Enter" => self.text_manager.text.clear(),
                _ => {
                    let chars = key.chars().collect::<Vec<_>>();
                    if chars.len() == 1 {
                        self.text_manager.add_char(chars[0])
                    }
                }
            }
        }
    }

    fn handle_window_resize(&mut self, event: &BEvent, new_size: Point) {
        println!("{:?}", event);
        let Point {
            x: width_pixels,
            y: height_pixels,
        } = new_size;
        let Config {
            height: height_tiles,
            width: width_tiles,
            tile_size_x,
            tile_size_y,
            ..
        } = self.config.borrow().clone();

        self.config.borrow_mut().scale_x =
            width_pixels as f32 / width_tiles as f32 / tile_size_x as f32;
        self.config.borrow_mut().scale_y =
            height_pixels as f32 / height_tiles as f32 / tile_size_y as f32;
    }

    fn handle_flake_text_manager(&mut self, ctx: &mut BTerm, fancy_batch: &mut DrawBatch) {
        // Add flakes if required
        let required_flake_count = self.text_manager.required_flakes();
        if required_flake_count > 0 {
            let flake_man = &mut self.snowflake_manager;
            let max_flakes_to_remove = std::usize::MAX
                .min(flake_man.snowflakes.len())
                .min(required_flake_count as usize);

            let mut flakes = Vec::new();
            for _ in 0..max_flakes_to_remove {
                let random_flake_index =
                    rand::thread_rng().gen_range(0..flake_man.snowflakes.len());
                flakes.push(flake_man.snowflakes.swap_remove(random_flake_index));
            }
            self.text_manager.add_flakes(flakes);
        }

        self.snowflake_manager.tick(ctx, fancy_batch);
    }

    fn handle_solution_progress_updates(&mut self) {
        if let Some(x) = &self.solve_state {
            if let Some(items) = x.lock().unwrap().next_items() {
                for item in items {
                    self.text_manager.text.clear();
                    let line = format!("{:?}\n", &item);
                    line.chars().for_each(|c| self.text_manager.add_char(c));
                }
            }
        }
    }

    /// Apply config changes from javascript if we are running in WASM
    fn handle_js_scale_changes(&mut self, normal_batch: &mut DrawBatch) {
        let js_bridge = js_interop::JS_BRIDGE.lock().unwrap();
        if js_bridge.scale > 0.0 {
            normal_batch.print(
                Point::from_tuple((1, 5)),
                format!("Scale: {:.3}", js_bridge.scale),
            );
            (*self.config.borrow_mut()).scale_x = js_bridge.scale as f32;
            (*self.config.borrow_mut()).scale_y = js_bridge.scale as f32;
        }
    }
}
