use crate::config::Config;
use bracket_terminal::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

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
            + (self.base.x + self.elapsed / 300.0 * self.v_sin_x).sin() * self.d_sin_x
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

pub struct SnowflakeManager {
    snowflakes: Vec<Snowflake>,
    config: Config,
}
impl SnowflakeManager {
    pub fn new(config: Config) -> Self {
        SnowflakeManager {
            config,
            snowflakes: Default::default(),
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        let Config { width, height, .. } = self.config;
        let mut rng = rand::thread_rng();
        let width_die = Uniform::from(0.0..width as f32);
        let height_die = Uniform::from(0.0..1.0);
        let height_starter_die = Uniform::from(0.0..height as f32);
        let snowflakes_count = 400;

        self.snowflakes.retain(|x| !x.done);
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
        let mp = INPUT.lock().mouse_tile(0);
        for flake in self.snowflakes.iter_mut() {
            let d = DistanceAlg::Pythagoras.distance2d(flake.pos.into(), mp);
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
            ..Default::default()
        });
    }
}
