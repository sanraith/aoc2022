use super::animator::{AnimationState, Animator, AnimatorBase};
use crate::{drawing::drawing_base::Drawable, util::distance2d_pythagoras_f32};
use bracket_terminal::prelude::{BTerm, PointF};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct MoveToAnimator<T: Drawable> {
    base: AnimatorBase,
    target: Rc<RefCell<T>>,
    end_pos: PointF,
    end_delta: f32,
    v: f32,
}
impl<T: Drawable> Animator for MoveToAnimator<T> {
    fn tick(&mut self, ctx: &BTerm) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;
        let mut target_ = self.target.borrow_mut();
        let mut target = target_.base_mut();

        let remaining_x = target.pos.x - self.end_pos.x;
        let remaining_y = target.pos.y - self.end_pos.y;
        target.pos.x += (self.v * elapsed_seconds).min(remaining_x);
        target.pos.y += (self.v * elapsed_seconds).min(remaining_y);
    }

    fn state(&self) -> AnimationState {
        let target_ = self.target.borrow();
        let target = target_.base();
        match distance2d_pythagoras_f32(&target.pos, &self.end_pos) <= self.end_delta {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }
}

impl<T: Drawable> Deref for MoveToAnimator<T> {
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T: Drawable> DerefMut for MoveToAnimator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
