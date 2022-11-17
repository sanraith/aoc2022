use super::animator::{AnimationState, Animator};
use crate::{drawing::drawing_base::Drawable, util::distance2d_pythagoras_f32};
use bracket_terminal::prelude::{BTerm, PointF};

pub struct MoveToAnimator {
    end_pos: PointF,
    end_delta: f32,
    v: f32,
    total_elapsed: f32,
    state: AnimationState,
}
impl<T: Drawable> Animator<T> for MoveToAnimator {
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;
        let mut target = target.base_mut();

        let remaining_x = target.pos.x - self.end_pos.x;
        let remaining_y = target.pos.y - self.end_pos.y;
        target.pos.x += (self.v * elapsed_seconds).min(remaining_x);
        target.pos.y += (self.v * elapsed_seconds).min(remaining_y);

        self.state = match distance2d_pythagoras_f32(&target.pos, &self.end_pos) <= self.end_delta {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        };
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
