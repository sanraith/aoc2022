use crate::{
    config::{self, Config},
    state::UiState,
};
use bracket_terminal::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let config = config::default();
    let Config {
        width,
        height,
        tile_size_x,
        tile_size_y,
    } = config;

    let context = BTermBuilder::simple(width, height)
        .expect("simple terminal should build")
        .with_title("Advent of Code 2022 by Soma Zsják")
        .with_fps_cap(60.0)
        .with_dimensions(width, height)
        .with_tile_dimensions(tile_size_x, tile_size_y)
        .with_fancy_console(width, height, "terminal8x8.png")
        .build()
        .expect("terminal should build");

    let gs = UiState::new(&config);
    main_loop(context, gs).unwrap();
}
