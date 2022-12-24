use crate::{
    char_image,
    config::Config,
    js_interop::{self, TouchKind, JS_BRIDGE},
    manager::{
        flake_text_manager::FlakeCharLine, snowflake_manager::SnowflakeManager,
        ui_text_manager::UiTextManager,
    },
    util::get_mouse_tile_pos,
    wasm_runner::WasmRunner,
};
use aoc::{
    core::solution_runner::{
        LocalSyncStream, SolutionRunner, SolveProgress, SyncStream, ThreadSolutionRunner,
    },
    solutions,
    util::YearDay,
};
use bracket_terminal::prelude::*;
use itertools::Itertools;
use rand::Rng;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

enum SolveState {
    NotSolved,
    Solving,
    Solved,
}

pub struct UiState {
    config: Rc<RefCell<Config>>,
    total_time: f32,
    snowflake_manager: Rc<RefCell<SnowflakeManager>>,
    text_manager: FlakeCharLine,
    solve_stream: Option<Arc<Mutex<LocalSyncStream>>>,
    ui_text_manager: UiTextManager,
    solve_state: SolveState,
}
impl GameState for UiState {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.total_time += ctx.frame_time_ms;
        let bg_color = (15, 15, 35, 255);
        ctx.cls_bg(bg_color);
        ctx.screen_burn_color(RGB::named(CYAN));

        let mut normal_batch = DrawBatch::new();
        normal_batch.target(0);
        normal_batch.cls_color(bg_color);

        let mut fancy_batch = DrawBatch::new();
        fancy_batch.target(1);
        fancy_batch.cls_color(bg_color);

        self.handle_js_scale_changes();
        self.handle_bracket_events(ctx);
        self.handle_mouse(&mut fancy_batch);

        self.handle_solution_progress_updates();
        self.handle_flake_text_manager(ctx, &mut fancy_batch);
        self.text_manager.tick(ctx, &mut fancy_batch);
        self.ui_text_manager.tick(ctx, &mut normal_batch);
        self.print_status(ctx, &mut normal_batch);

        fancy_batch.submit(2).expect("Render error");
        normal_batch.submit(1).expect("Render error");
        render_draw_buffer(ctx).expect("Render error");
    }
}
impl UiState {
    pub fn new(config: Config) -> Self {
        let config = Rc::from(RefCell::new(config));
        let snowflake_manager = Rc::new(RefCell::new(SnowflakeManager::new(Rc::clone(&config))));
        UiState {
            config: Rc::clone(&config),
            snowflake_manager: Rc::clone(&snowflake_manager),
            text_manager: FlakeCharLine::new(
                PointF::from((1.0, 8.0)),
                2000.0,
                1000.0,
                500.0,
                (255, 255, 255, 255),
            ),
            total_time: 0.0,
            solve_stream: None,
            ui_text_manager: UiTextManager::new(config, snowflake_manager, Point::new(1, 5)),
            solve_state: SolveState::NotSolved,
        }
    }

    fn print_status(&self, ctx: &BTerm, batch: &mut DrawBatch) {
        batch.print_color_centered(
            2,
            "*** Advent of Code 2022 ***",
            // ColorPair::new((255, 255, 255, 255), (0, 0, 0, 0)), // white
            // ColorPair::new((127, 189, 57, 255), (0, 0, 0, 0)), // yellow
            ColorPair::new((0, 204, 0, 255), (0, 0, 0, 0)), // AOC bright green
        );
        let status = format!("FPS: {: >6}", ctx.fps as i32);
        let status_color = ColorPair::new((216, 216, 216, 255), (0, 0, 0, 0)); // gray
        batch.print_color(Point::from_tuple((78, 1)), status, status_color);
        let js_bridge = JS_BRIDGE.lock().unwrap();
        if js_bridge.scale > 0.0 && (js_bridge.scale - 1.0).abs() > 0.00001 {
            let scale = &format!("Scale: {:.2}", js_bridge.scale);
            batch.print_color(Point::from_tuple((78, 2)), scale, status_color);
        }

        // Print start button
        if let SolveState::NotSolved = self.solve_state {
            let button_lines = char_image::draw_text("Tap here", '#', ' ')
                .into_iter()
                .map(|l| format!("   {}", l))
                .chain("\n".split("\n").map(|x| x.to_owned()))
                .chain(char_image::draw_text("to start!", '#', ' ').into_iter())
                .collect_vec();
            let x_start = (self.config.borrow().width as i32
                - button_lines.iter().map(|l| l.len()).max().unwrap() as i32)
                / 2;
            let y_start = (self.config.borrow().height as i32 - button_lines.len() as i32) / 2;
            let button_lines = button_lines
                .into_iter()
                .map(|l| {
                    l.chars()
                        .map(|c| if c == '#' { '█' } else { c })
                        .collect::<String>()
                })
                .collect_vec();
            for (y, line) in button_lines.iter().enumerate() {
                batch.print_color(
                    Point::new(x_start, y_start + y as i32),
                    line,
                    ColorPair::new((255, 255, 255, 255), (0, 0, 0, 0)),
                );
            }
        }
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

        // Start solver via mouse click
        if INPUT.lock().is_mouse_button_pressed(0) && self.solve_stream.is_none() {
            self.start_solving_solutions();
        }

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

        let mouse_pos = INPUT.lock().mouse_pixel_pos();
        match mouse_pos {
            (x, y) if x <= 0.0 && y <= 0.0 => (),
            (x, y) => self.config.borrow_mut().mouse = (x as f32, y as f32),
        }
        let js_unhandled_touches = JS_BRIDGE
            .lock()
            .unwrap()
            .unhandled_touches
            .drain(..)
            .collect_vec();
        for (p, kind) in js_unhandled_touches {
            self.config.borrow_mut().mouse = (p[0], p[1]);
            if let TouchKind::End = kind {
                if self.solve_stream.is_none() {
                    self.start_solving_solutions();
                }
            }
        }
    }

    fn start_solving_solutions(&mut self) {
        self.ui_text_manager.clear();
        self.solve_state = SolveState::Solving;
        let runner: Box<dyn SolutionRunner<LocalSyncStream>> =
            match JS_BRIDGE.lock().unwrap().worker_wrapper {
                Some(_) => Box::new(WasmRunner {}),
                None => Box::new(ThreadSolutionRunner {}),
            };
        self.solve_stream = Some(runner.run(
            YearDay { year: 2022, day: 1 },
            aoc::core::solution_runner::Input::Default,
        ));
    }

    fn handle_window_resize(&mut self, event: &BEvent, new_size: Point) {
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
        let mut flake_man = self.snowflake_manager.borrow_mut();
        if required_flake_count > 0 {
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

        flake_man.tick(ctx, fancy_batch);
    }

    fn handle_solution_progress_updates(&mut self) {
        let mut next_year: Option<YearDay> = None;
        if let Some(x) = &self.solve_stream {
            if let Some(items) = x.lock().unwrap().next_items() {
                for item in items {
                    if let SolveProgress::Done(pack) = &item {
                        let map = solutions::create_map();
                        let max_day = map.keys().max();
                        if let Some(max_day) = max_day {
                            if pack.year_day < *max_day {
                                next_year =
                                    Some(YearDay::new(pack.year_day.year, pack.year_day.day + 1));
                            } else {
                                self.solve_state = SolveState::Solved;
                            }
                        }
                    }
                    self.ui_text_manager.update_progress(item);
                }
            }
        }

        if let SolveState::Solved = self.solve_state {
            self.solve_stream = None;
        }

        // Start next day if there are more days to solve
        if let Some(mut next_day) = next_year {
            // TODO remove if d16 implementation is faster
            if next_day.day == 16 {
                next_day.day = 17;
            }
            let runner: Box<dyn SolutionRunner<LocalSyncStream>> =
                match JS_BRIDGE.lock().unwrap().worker_wrapper {
                    Some(_) => Box::new(WasmRunner {}),
                    None => Box::new(ThreadSolutionRunner {}),
                };
            self.solve_stream =
                Some(runner.run(next_day, aoc::core::solution_runner::Input::Default));
        }
    }

    /// Apply config changes from javascript if we are running in WASM
    fn handle_js_scale_changes(&mut self) {
        let js_bridge = js_interop::JS_BRIDGE.lock().unwrap();
        if js_bridge.scale > 0.0 {
            (*self.config.borrow_mut()).scale_x = js_bridge.scale as f32;
            (*self.config.borrow_mut()).scale_y = js_bridge.scale as f32;
        }
    }
}
