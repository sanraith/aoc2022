use bracket_terminal::prelude::*;
use wasm_bindgen::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}

#[wasm_bindgen]
pub fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()
        .unwrap();

    let gs: State = State {};
    main_loop(context, gs).unwrap();
}
