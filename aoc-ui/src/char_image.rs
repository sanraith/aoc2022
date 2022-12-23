use crate::resources;
use bracket_terminal::prelude::to_char;
use image::{io::Reader, GenericImageView};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{collections::HashMap, io::Cursor};

pub const CHAR_HEIGHT: u8 = 8;
pub const CHAR_WIDTH: u8 = 8;
pub static CHARACTER_IMAGES: Lazy<HashMap<char, CharImage>> = Lazy::new(load_character_map);

pub struct CharImage {
    pub texture_index: usize,
    pub char: char,
    pub rows: [[u8; CHAR_WIDTH as usize]; CHAR_WIDTH as usize],
}

pub fn draw_text(text: &str, on: char, off: char) -> Vec<String> {
    let mut lines = Vec::with_capacity(CHAR_HEIGHT as usize);
    (0..CHAR_HEIGHT)
        .into_iter()
        .for_each(|_| lines.push(Vec::with_capacity(text.len() * CHAR_WIDTH as usize)));

    for char in text.chars() {
        let char_image = CHARACTER_IMAGES
            .get(&char)
            .or_else(|| CHARACTER_IMAGES.get(&'?'))
            .unwrap();
        for (row_index, row) in char_image.rows.iter().enumerate() {
            for &pixel in row {
                let out_char = if pixel > 127 { on } else { off };
                lines[row_index].push(out_char);
            }
        }
    }

    lines
        .into_iter()
        .map(|x| x.into_iter().collect::<String>())
        .collect_vec()
}

/// Prints a line by converting each character pixel to a single character itself
pub fn print_text(text: &str, on: char, off: char) {
    let lines = draw_text(text, on, off);
    for line in lines {
        println!("{}", line);
    }
}

fn load_character_map() -> HashMap<char, CharImage> {
    const _MAP_HEIGHT: u32 = 16;
    const MAP_WIDTH: u32 = 16;

    let cursor = Cursor::new(resources::FONT_TERMINAL8X8_PNG);
    let mut reader = Reader::new(cursor);
    reader.set_format(image::ImageFormat::Png);
    let img = reader.decode().expect("image should decode");
    let mut chars = [[[0 as u8; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize]; 256];
    for (x, y, c) in img.pixels() {
        let char_index = (x / CHAR_WIDTH as u32 + y / CHAR_HEIGHT as u32 * MAP_WIDTH) as usize;
        let y_local = (y % CHAR_HEIGHT as u32) as usize;
        let x_local = (x % CHAR_HEIGHT as u32) as usize;
        chars[char_index][y_local][x_local] = c[1];
    }

    let mut character_map = HashMap::new();
    for (index, image_bytes) in chars.into_iter().enumerate() {
        let character = to_char(index as u8);
        character_map.insert(
            character,
            CharImage {
                texture_index: index,
                char: character,
                rows: image_bytes,
            },
        );
    }
    character_map
}
