#[derive(Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub tile_size_x: u32,
    pub tile_size_y: u32,
    pub scale_x: f32,
    pub scale_y: f32,
}

pub fn default() -> Config {
    Config {
        width: 90,
        height: 50,
        tile_size_x: 16,
        tile_size_y: 16,
        scale_x: 1.0,
        scale_y: 1.0,
    }
}
