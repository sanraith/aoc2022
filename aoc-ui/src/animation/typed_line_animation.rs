use super::{
    animator::AnimationState,
    ease::{ease_progress, EaseType},
};
use crate::drawing::drawing_base::DrawingBase;
use bracket_terminal::prelude::*;

pub struct TypedLineAnimation {
    pub base: DrawingBase,
    pub text: String,
    pub anim_length_ms: f32,
    pub ease_type: EaseType,
    pub elapsed: f32,
    pub bg_color: (u8, u8, u8, u8),
}

impl TypedLineAnimation {
    pub fn new(
        base: DrawingBase,
        text: String,
        anim_length_ms: f32,
        ease_type: EaseType,
        bg_color: (u8, u8, u8, u8),
    ) -> TypedLineAnimation {
        TypedLineAnimation {
            base,
            text,
            anim_length_ms,
            ease_type,
            elapsed: 0.0,
            bg_color,
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        self.elapsed += ctx.frame_time_ms;

        let eased_progress = ease_progress(self.progress(), &self.ease_type);
        let drawn_text = match self.progress() {
            p if p >= 1.0 => self.text.to_owned(),
            _ => {
                self.text
                    .chars()
                    .take((eased_progress * self.text.len() as f32) as usize)
                    .collect::<String>()
                    + "█"
            }
        };
        batch.print_color(
            Point::new(self.base.pos.x as i32, self.base.pos.y as i32),
            drawn_text,
            ColorPair::new(self.base.color, self.bg_color),
        );
    }

    pub fn progress(&self) -> f32 {
        (self.elapsed / self.anim_length_ms).min(1.0)
    }

    pub fn state(&self) -> AnimationState {
        match self.elapsed.total_cmp(&self.anim_length_ms) {
            std::cmp::Ordering::Less => AnimationState::Running,
            _ => AnimationState::Completed,
        }
    }
}
