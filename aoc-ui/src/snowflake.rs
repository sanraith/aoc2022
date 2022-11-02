use crate::config::Config;
use bracket_terminal::prelude::*;
use rand::{distributions::Uniform, prelude::Distribution, Rng};

const SNOWFLAKE_COLOR: (u8, u8, u8, u8) = (255, 255, 255, 200);

struct Snowflake {
    base: PointF,
    pos: PointF,
    vx: f32,
    vy: f32,
    max_y: f32,
    elapsed: f32,
    done: bool,
}
impl Snowflake {
    pub fn new(base: PointF, vx: f32, vy: f32, max_y: f32) -> Self {
        Self {
            base,
            vx,
            vy,
            max_y,
            pos: PointF::default(),
            elapsed: 0.0,
            done: false,
        }
    }

    pub fn progress(&mut self, ctx: &BTerm) {
        self.elapsed += ctx.frame_time_ms;
        self.pos.y = self.base.y + self.elapsed * self.vy / 1000.0;
        self.pos.x = self.base.x + (self.base.x + self.elapsed / 300.0).sin() * self.vx;
        if self.pos.y >= self.max_y {
            self.done = true;
        }
    }

    pub fn draw(&self, batch: &mut DrawBatch) {
        batch.set_fancy(
            self.pos.clone(),
            0,
            Degrees::new(0.0),
            PointF::new(0.3, 0.5),
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
        let snowflakes_count = 200;

        self.snowflakes.retain(|x| !x.done);
        let height_die = match self.snowflakes.len() {
            0 => height_starter_die, // distribute flakes vertically initially
            _ => height_die,         // spawn new ones at the top
        };
        for _ in 0..snowflakes_count - self.snowflakes.len() {
            self.create_snowflake(&mut rng, width_die, height_die, height);
        }

        for flake in self.snowflakes.iter_mut() {
            flake.progress(ctx);
            flake.draw(batch)
        }
    }

    fn create_snowflake(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        width_die: Uniform<f32>,
        height_die: Uniform<f32>,
        max_y: u32,
    ) {
        self.snowflakes.push(Snowflake::new(
            PointF {
                x: width_die.sample(rng),
                y: height_die.sample(rng),
            },
            rng.gen_range(0.1..1.0),
            rng.gen_range(2.0..10.0),
            max_y as f32,
        ));
    }
}
