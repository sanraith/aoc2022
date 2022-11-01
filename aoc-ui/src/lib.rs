use bracket_terminal::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Default)]
struct State {
    pub total_time: f32,
}

const WIDTH: i32 = 90;
const HEIGHT: i32 = 50;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.total_time += ctx.frame_time_ms;
        let mut batch = DrawBatch::new();
        batch.cls();

        let status = format!(
            "{:>3} FPS, runtime: {}s",
            ctx.fps as i32,
            (self.total_time / 1000.0) as i32
        );
        batch.print(Point::from_tuple((1, 2)), status);

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

        let p = INPUT.lock().mouse_tile(0);
        let p = Point {
            x: p.x + 1,
            y: p.y + 1,
        };
        batch.print(p, ".");

        batch.submit(0).expect("Render error");
        render_draw_buffer(ctx).expect("Render error");
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
