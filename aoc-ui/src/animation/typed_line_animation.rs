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
}

impl TypedLineAnimation {
    pub fn new(
        base: DrawingBase,
        text: String,
        anim_length_ms: f32,
        ease_type: EaseType,
    ) -> TypedLineAnimation {
        TypedLineAnimation {
            base,
            text,
            anim_length_ms,
            ease_type,
            elapsed: 0.0,
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        self.elapsed += ctx.frame_time_ms;

        let eased_progress = ease_progress(self.progress(), &self.ease_type);
        let drawn_text = self
            .text
            .chars()
            .take((eased_progress * self.text.len() as f32) as usize)
            .collect::<String>();
        batch.print_color(
            Point::new(self.base.pos.x as i32, self.base.pos.y as i32),
            drawn_text,
            ColorPair::new(self.base.color, (0, 0, 0, 0)),
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
