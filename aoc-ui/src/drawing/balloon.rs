use super::drawing_base::{Drawable, DrawingBase};
use bracket_terminal::prelude::*;
use once_cell::sync::Lazy;

static BALLOON: Lazy<Vec<Vec<char>>> = Lazy::new(|| create_balloon());

#[derive(Clone)]
pub struct Balloon {
    pub base: DrawingBase,
}

impl Drawable for Balloon {
    fn draw(&self, _ctx: &BTerm, batch: &mut DrawBatch) {
        for (y, line) in BALLOON.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == ' ' {
                    continue;
                }
                self.draw_char(
                    batch,
                    *c,
                    PointF::new(x as f32 * self.base.scale * 0.5, y as f32 * self.base.scale),
                );
            }
        }
    }

    fn base(&self) -> &DrawingBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut DrawingBase {
        &mut self.base
    }
}

impl Balloon {
    fn draw_char(&self, batch: &mut DrawBatch, ch: char, p: PointF) {
        let c = self.base.color;
        batch.set_fancy(
            self.base.pos + p,
            self.base.z_order,
            Degrees::new(self.base.rotation),
            PointF::new(self.base.scale * 0.5, self.base.scale),
            ColorPair::new(
                RGBA::from_u8(c.0, c.1, c.2, (self.base.opaqueness * 255.0) as u8),
                RGBA::from_u8(0, 0, 0, 0),
            ),
            to_cp437(ch),
        );
    }
}

fn create_balloon() -> Vec<Vec<char>> {
    let balloon = r#"
    _..----..
  .'_|` _|` _'.
 /_|  _|  _|  _\
;|  _|  _|  _| ||
| _| _|  _|  _| |
||  |  _|  _|  _|
 \_| _|  _|  _|/
  `.|  _|  _|.`
   \ )_|  _| /
    '\|__|__/
      ;____;
       \YT/
        ||
       |""|
       '=='"#;
    balloon
        .lines()
        .skip(1)
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
