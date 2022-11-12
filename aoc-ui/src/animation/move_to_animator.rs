use super::animator::{Animator, AnimatorBase};
use crate::{drawing::drawing_base::DrawingBase, util::distance2d_pythagoras_f32};
use bracket_terminal::prelude::{BTerm, PointF};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct MoveToAnimator {
    base: AnimatorBase,
    target: Rc<RefCell<DrawingBase>>,
    end_pos: PointF,
    end_delta: f32,
    v: f32,
}
impl Animator<DrawingBase> for MoveToAnimator {
    fn tick(&mut self, ctx: &BTerm) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;
        let mut target = self.target.borrow_mut();

        let remaining_x = target.pos.x - self.end_pos.x;
        let remaining_y = target.pos.y - self.end_pos.y;
        target.pos.x += (self.v * elapsed_seconds).min(remaining_x);
        target.pos.y += (self.v * elapsed_seconds).min(remaining_y);
    }

    fn is_completed(&self) -> bool {
        let target = self.target.borrow();
        distance2d_pythagoras_f32(&target.pos, &self.end_pos) <= self.end_delta
    }

    fn get_target(&self) -> Rc<RefCell<DrawingBase>> {
        Rc::clone(&self.target)
    }
}

impl Deref for MoveToAnimator {
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for MoveToAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
