use crate::{solution::*, solutions::Day01};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
    log(&format!("Hello {}!", name));

    let ctx = Context {
        raw_input: "123\n456\n789".to_owned(),
        ..Default::default()
    };
    let mut day = Day01::new();
    let result = day.part1(&ctx).unwrap();
    log(&format!("Solution {}!", result));
}
