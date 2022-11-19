use super::animator::{AnimationState, Animator};
use crate::{drawing::drawing_base::Drawable, util::distance2d_pythagoras_f32};
use bracket_terminal::prelude::{BTerm, PointF};

pub struct MoveToAnimator {
    pub end_pos: PointF,
    pub end_delta: f32,
    pub v: f32,
    pub total_elapsed: f32,
    pub state: AnimationState,
}
impl<T: Drawable> Animator<T> for MoveToAnimator {
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_s = ctx.frame_time_ms / 1000.0;
        let mut target = target.base_mut();

        let remaining_x = self.end_pos.x - target.pos.x;
        let remaining_y = self.end_pos.y - target.pos.y;
        let ratio_x = remaining_x.abs() / (remaining_x + remaining_y).abs();
        let ratio_y = remaining_y.abs() / (remaining_x + remaining_y).abs();

        target.pos.x +=
            (ratio_x * self.v * elapsed_s).min(remaining_x.abs()) * remaining_x.signum();
        target.pos.y +=
            (ratio_y * self.v * elapsed_s).min(remaining_y.abs()) * remaining_y.signum();

        self.state = match distance2d_pythagoras_f32(&target.pos, &self.end_pos) <= self.end_delta {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        };
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
