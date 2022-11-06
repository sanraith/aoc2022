use crate::{
    char_image::{self, CHARACTER_IMAGES},
    config::Config,
    util::{distance2d_pythagoras_f32, get_mouse_tile_pos},
};
use bracket_terminal::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use std::{cell::RefCell, rc::Rc};

const SNOWFLAKE_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 200);

#[derive(Default)]
struct Snowflake {
    base: PointF,
    d_sin_x: f32,
    v_sin_x: f32,
    vx: f32,
    vx_extra: f32,
    vy: f32,
    vy_extra: f32,
    rot: f32,
    v_rot: f32,
    scale: f32,
    max_x: f32,
    max_y: f32,

    /// Used to spice up sin() output for different y values
    seed: f32,
    pos: PointF,
    elapsed: f32,
    done: bool,
}
impl Snowflake {
    pub fn progress(&mut self, ctx: &BTerm) {
        self.elapsed += ctx.frame_time_ms;
        let elapsed_seconds = self.elapsed / 1000.0;
        self.pos.y = self.base.y + elapsed_seconds * self.vy;

        self.pos.x = (self.base.x
            + (self.seed + self.elapsed / 300.0 * self.v_sin_x).sin() * self.d_sin_x
            + elapsed_seconds * self.vx)
            % self.max_x
            - 0.5;
        self.base.x += ctx.frame_time_ms / 1000.0 * self.vx_extra;
        self.base.y += ctx.frame_time_ms / 1000.0 * self.vy_extra;
        self.rot = elapsed_seconds * self.v_rot;
        if self.pos.y >= self.max_y {
            self.done = true;
        }
    }

    pub fn draw(&self, batch: &mut DrawBatch) {
        batch.set_fancy(
            self.pos.clone(),
            0,
            Degrees::new(self.rot),
            PointF::new(self.scale * 0.5, self.scale * 1.0),
            ColorPair::new(SNOWFLAKE_COLOR, RGBA::from_u8(0, 0, 0, 0)),
            to_cp437('*'),
        );
    }
}

struct TargetedSnowflake {
    flake: Snowflake,
    target: FlakeTarget,
    reached_target: bool,
}

struct FlakeTarget {
    pos: PointF,
    char_idx: usize,
}

pub struct SnowflakeManager {
    snowflakes: Vec<Snowflake>,
    config: Rc<RefCell<Config>>,

    text: Vec<char>,
    text_flakes: Vec<TargetedSnowflake>,
    text_flake_queue: Vec<FlakeTarget>,
}
impl SnowflakeManager {
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        SnowflakeManager {
            config,
            snowflakes: Default::default(),
            text: Default::default(),
            text_flakes: Default::default(),
            text_flake_queue: Default::default(),
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        let Config { width, height, .. } = *self.config.borrow();
        let mut rng = rand::thread_rng();
        let width_die = Uniform::from(0.0..width as f32);
        let height_die = Uniform::from(0.0..1.0);
        let height_starter_die = Uniform::from(0.0..height as f32);
        let snowflakes_count = 400;

        self.snowflakes.retain(|x| !x.done);

        self.handle_input(batch);
        self.handle_input_flakes(ctx, batch);

        let height_die = match self.snowflakes.len() {
            0 => height_starter_die, // distribute flakes vertically initially
            _ => height_die,         // spawn new ones at the top
        };
        for _ in 0..snowflakes_count - self.snowflakes.len() {
            self.create_snowflake(&mut rng, width_die, height_die, width, height);
        }

        let mouse_active = match INPUT.lock().mouse_pixel_pos() {
            (x, y) if x > 0.0 && y > 0.0 => true,
            _ => false,
        };
        let influence = 10.0;
        let power = 20.0;
        let mp = get_mouse_tile_pos(&self.config.borrow());
        for flake in self.snowflakes.iter_mut() {
            let d = distance2d_pythagoras_f32(&flake.pos, &mp);
            let direction_x = if flake.pos.x > mp.x as f32 { 1 } else { -1 } as f32;
            let direction_y = if flake.pos.y > mp.y as f32 { 1 } else { -1 } as f32;
            if mouse_active && d < influence {
                flake.vx_extra = ((influence - d) / influence) * power * direction_x;
                flake.vy_extra = ((influence - d) / influence) * power * direction_y;
            } else {
                flake.vx_extra = 0.0;
                flake.vy_extra = 0.0;
            }

            flake.progress(ctx);
            flake.draw(batch)
        }
    }

    fn handle_input(&mut self, batch: &mut DrawBatch) {
        let text_base_x = 1;
        let text_base_y = 10;

        INPUT.lock().for_each_message(|event| {
            match event {
                BEvent::KeyboardInput {
                    key: VirtualKeyCode::Back,
                    pressed: true,
                    ..
                } => {
                    self.text.pop();
                }
                BEvent::KeyboardInput {
                    key: VirtualKeyCode::Return,
                    pressed: true,
                    ..
                } => {
                    self.text.clear();
                    self.text_flake_queue.clear();
                    self.text_flakes.clear();
                }
                BEvent::Character { c } if char_image::CHARACTER_IMAGES.contains_key(&c) => {
                    self.text.push(c);
                    let image = CHARACTER_IMAGES
                        .get(&c)
                        .or_else(|| CHARACTER_IMAGES.get(&' '))
                        .expect("character image available");

                    let pixels = image.rows.iter().enumerate().flat_map(|(y, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(_, &pixel)| pixel > 127)
                            .map(move |(x, _)| (x, y))
                    });
                    for (x, y) in pixels {
                        self.text_flake_queue.push(FlakeTarget {
                            pos: PointF {
                                x: text_base_x as f32
                                    + (self.text.len() - 1) as f32
                                    + x as f32 / char_image::CHAR_WIDTH as f32
                                    - 0.5,
                                y: text_base_y as f32
                                    + y as f32 / char_image::CHAR_HEIGHT as f32
                                    + 0.5,
                            },
                            char_idx: self.text.len() - 1,
                        })
                    }
                }
                _ => (),
            };
        });

        for (idx, c) in self.text.iter().enumerate() {
            let is_waiting = self
                .text_flakes
                .iter()
                .any(|f| f.target.char_idx == idx && !f.reached_target)
                || self.text_flake_queue.iter().any(|f| f.char_idx == idx);
            if !is_waiting {
                batch.print(
                    Point::from_tuple((text_base_x + idx, text_base_y)),
                    c.to_string(),
                );
                self.text_flakes.retain(|f| f.target.char_idx != idx);
            }
        }
    }

    fn handle_input_flakes(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        let influence = 50.0;
        while let Some(target) = self.text_flake_queue.first() {
            if let Some((idx, ..)) = self
                .snowflakes
                .iter()
                .enumerate()
                .filter(|(_, a)| distance2d_pythagoras_f32(&target.pos, &a.pos) <= influence)
                // .into_sorted_by(|(_, a), (_, b)| {
                //     distance2d_pythagoras_f32(&target.pos, &a.pos)
                //         .partial_cmp(&distance2d_pythagoras_f32(&target.pos, &b.pos))
                //         .unwrap()
                // })
                .collect::<Vec<_>>()
                .first()
            {
                let mut flake = self.snowflakes.remove(*idx);
                let target = self.text_flake_queue.remove(0);
                flake.vy = 0.0; // TODO this resets the position, rewrite flakes to be more mutable
                flake.vx = 0.0;
                flake.d_sin_x = 0.1;
                flake.v_sin_x = 0.1;
                flake.scale = 0.35;
                self.text_flakes.push(TargetedSnowflake {
                    flake,
                    target,
                    reached_target: false,
                });
            } else {
                break;
            }
        }

        let power = 2.0;
        // let power_min = 5.0;
        for targeted in self.text_flakes.iter_mut() {
            let flake = &mut targeted.flake;
            let target = &targeted.target.pos;

            flake.vx_extra = ((target.x - flake.pos.x) * power)
                // .min(power_min)
                .max(target.x - flake.pos.x);
            flake.vy_extra = ((target.y - flake.pos.y) * power)
                // .min(power_min)
                .max(target.y - flake.pos.y);

            flake.progress(ctx);
            flake.draw(batch);

            if distance2d_pythagoras_f32(&flake.pos, &target) < 0.1 {
                targeted.reached_target = true;
            }
        }
    }

    fn create_snowflake(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        width_die: Uniform<f32>,
        height_die: Uniform<f32>,
        max_x: u32,
        max_y: u32,
    ) {
        self.snowflakes.push(Snowflake {
            base: PointF {
                x: width_die.sample(rng),
                y: height_die.sample(rng),
            },
            d_sin_x: rng.gen_range(0.1..1.0),
            v_sin_x: rng.gen_range(0.2..0.7),
            vx: rng.gen_range(-1.5..1.5),
            vy: rng.gen_range(2.0..8.0),
            max_x: max_x as f32,
            max_y: max_y as f32 + 1.0,
            rot: rng.gen_range(0.0..180.0),
            v_rot: rng.gen_range(-180.0..180.0),
            scale: rng.gen_range(0.25..1.0),
            seed: rng.gen_range(0.0..1000.0),
            ..Default::default()
        });
    }
}
