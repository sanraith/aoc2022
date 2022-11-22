use crate::{
    config::{self, Config},
    state::UiState,
};
use bracket_terminal::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
pub struct JsConfig {
    pub scale: f64,
    pub unhandled_keys: Vec<String>,
}
pub static JS_CONFIG: Lazy<Mutex<JsConfig>> = Lazy::new(|| Mutex::new(Default::default()));

#[wasm_bindgen]
pub fn worker_inc(num: JsValue) -> JsValue {
    let number = num.as_f64().unwrap() + 1.0;
    JsValue::from_f64(number)
}

#[wasm_bindgen]
pub fn set_scale(scale: JsValue) {
    let scale = scale.as_f64().unwrap();
    JS_CONFIG.lock().unwrap().scale = scale;
}

#[wasm_bindgen]
pub fn push_key_event(key: JsValue) {
    let key = key.as_string().unwrap();
    JS_CONFIG.lock().unwrap().unhandled_keys.push(key);
}

#[wasm_bindgen]
pub fn main_wasm() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    main().map_err(|x| JsValue::from(format!("{:?}", x)))?;

    Ok(())
}

pub fn main() -> BResult<()> {
    let config = config::default();
    let Config {
        width,
        height,
        tile_size_x,
        tile_size_y,
        ..
    } = config;

    let context = BTermBuilder::simple(width, height)
        .expect("simple terminal should build")
        .with_title("Advent of Code 2022 by Soma Zsják")
        .with_dimensions(width, height)
        .with_tile_dimensions(tile_size_x, tile_size_y)
        .with_fancy_console(width, height, "terminal8x8.png")
        .with_fps_cap(144.0)
        .with_advanced_input(true)
        .build()?;

    let gs = UiState::new(config);
    main_loop(context, gs)?;

    Ok(())
}
