use super::animator::*;
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct TransitionAnimator {
    pub base: AnimatorBase,
    pub a: Box<dyn Animator>,
    pub b: Box<dyn Animator>,
    pub target: Rc<RefCell<Box<dyn Drawable>>>,
    pub length_ms: f32,
    pub elapsed: f32,
}

impl TransitionAnimator {
    pub fn new(
        a: Box<dyn Animator>,
        b: Box<dyn Animator>,
        target: Rc<RefCell<Box<dyn Drawable>>>,
        length_ms: f32,
    ) -> TransitionAnimator {
        let start_pos = target.borrow().base().pos.clone();
        TransitionAnimator {
            base: AnimatorBase {
                start_pos,
                ..Default::default()
            },
            a,
            b,
            target,
            length_ms,
            elapsed: 0.0,
        }
    }
}

impl Animator for TransitionAnimator {
    fn tick(&mut self, _ctx: &BTerm) {
        // self.elapsed += ctx.frame_time_ms;
        // self.a.tick(ctx);
        // self.b.tick(ctx);

        // let a_cell = self.a.get_target();
        // let mut a = a_cell.borrow_mut();
        // let b_cell = self.b.get_target();
        // let mut b = b_cell.borrow_mut();

        // let mut target = self.target.borrow_mut();
        // let phase_a = self.elapsed.min(self.length_ms) / self.length_ms;
        // let phase_b = 1.0 - phase_a;
        // target.pos.x = a.pos.x * phase_a + b.pos.x * phase_b;
        // target.pos.y = a.pos.y * phase_a + b.pos.y * phase_b;
        // target.rotation = a.rotation * phase_a + b.rotation * phase_b;
        // target.scale = a.scale * phase_a + b.scale * phase_b;
        // target.transparency = a.transparency * phase_a + b.transparency * phase_b;

        // a.visible = false;
        // b.visible = false;
    }

    fn state(&self) -> AnimationState {
        match self.elapsed >= self.length_ms {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }
}

impl Deref for TransitionAnimator {
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for TransitionAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
