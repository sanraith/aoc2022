use bracket_terminal::prelude::*;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use snowflake::Snowflake;
use wasm_bindgen::prelude::*;
mod snowflake;

#[derive(Default)]
struct State {
    pub total_time: f32,
    pub snowflakes: Vec<Snowflake>,
}

const WIDTH: i32 = 90;
const HEIGHT: i32 = 50;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.total_time += ctx.frame_time_ms;
        let mut batch = DrawBatch::new();
        batch.cls();

        self.handle_snowflakes(ctx, &mut batch);
        self.handle_status(ctx, &mut batch);
        self.handle_borders(&mut batch);
        self.handle_mouse(&mut batch);

        batch.submit(0).expect("Render error");
        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State {
    fn handle_status(&self, ctx: &mut BTerm, batch: &mut DrawBatch) {
        let status = format!(
            "{:>3} FPS, runtime: {}s",
            ctx.fps as i32,
            (self.total_time / 1000.0) as i32
        );
        batch.print(Point::from_tuple((1, 2)), status);
    }

    fn handle_mouse(&self, batch: &mut DrawBatch) {
        let p = INPUT.lock().mouse_tile(0);
        let p = Point {
            x: p.x + 1,
            y: p.y + 1,
        };
        batch.print(p, ".");
    }

    fn handle_borders(&mut self, batch: &mut DrawBatch) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x == 0 || y == 0 || x == WIDTH - 1 || y == HEIGHT - 1 {
                    if (y + x + ((self.total_time / 500.0) as i32 % 2)) % 2 == 0 {
                        batch.print(Point::from_tuple((x, y)), "*");
                    } else {
                        batch.print(Point::from_tuple((x, y)), ".");
                    }
                }
            }
        }
    }

    fn handle_snowflakes(&mut self, ctx: &mut BTerm, batch: &mut DrawBatch) {
        let mut rng = rand::thread_rng();
        let width_die = Uniform::from(0..WIDTH);
        let height_die = Uniform::from(0..5);

        self.snowflakes.retain(|x| !x.done);
        if self.snowflakes.len() < 100 {
            self.snowflakes.push(Snowflake {
                base: Point {
                    x: width_die.sample(&mut rng),
                    y: height_die.sample(&mut rng),
                },
                max_y: HEIGHT as f32,
                vx: rng.gen_range(0.1..1.5),
                vy: rng.gen_range(2.0..10.0),
                ..Default::default()
            })
        }

        for flake in self.snowflakes.iter_mut() {
            flake.progress(ctx);
            flake.draw(batch)
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    let context = BTermBuilder::simple(WIDTH, HEIGHT)
        .unwrap()
        .with_tile_dimensions(16, 16)
        .with_title("Advent of Code 2022 by Soma Zsják")
        .with_advanced_input(true)
        .build()
        .unwrap();

    let gs: State = State {
        ..Default::default()
    };
    main_loop(context, gs).unwrap();
}
