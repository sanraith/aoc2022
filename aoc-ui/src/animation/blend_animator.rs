use super::animator::*;
use crate::drawing::drawing_base::DrawingBase;
use bracket_terminal::prelude::BTerm;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct BlendAnimator<T> {
    pub base: AnimatorBase,
    pub a: Box<dyn Animator<T>>,
    pub b: Box<dyn Animator<T>>,
    pub target: Rc<RefCell<T>>,
    pub length_ms: f32,
    pub elapsed: f32,
}

impl BlendAnimator<DrawingBase> {
    pub fn new(
        a: Box<dyn Animator<DrawingBase>>,
        b: Box<dyn Animator<DrawingBase>>,
        target: Rc<RefCell<DrawingBase>>,
        length_ms: f32,
    ) -> BlendAnimator<DrawingBase> {
        let start_pos = target.borrow().pos.clone();
        BlendAnimator {
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

impl Animator<DrawingBase> for BlendAnimator<DrawingBase> {
    fn tick(&mut self, ctx: &BTerm) {
        self.elapsed += ctx.frame_time_ms;
        self.a.tick(ctx);
        self.b.tick(ctx);

        let a_cell = self.a.get_target();
        let mut a = a_cell.borrow_mut();
        let b_cell = self.b.get_target();
        let mut b = b_cell.borrow_mut();

        let mut target = self.target.borrow_mut();
        let phase_a = self.elapsed.min(self.length_ms) / self.length_ms;
        let phase_b = 1.0 - phase_a;
        target.pos.x = a.pos.x * phase_a + b.pos.x * phase_b;
        target.pos.y = a.pos.y * phase_a + b.pos.y * phase_b;
        target.rotation = a.rotation * phase_a + b.rotation * phase_b;
        target.scale = a.scale * phase_a + b.scale * phase_b;
        target.transparency = a.transparency * phase_a + b.transparency * phase_b;

        a.visible = false;
        b.visible = false;
    }

    fn is_completed(&self) -> bool {
        self.elapsed >= self.length_ms
    }

    fn get_target(&self) -> Rc<RefCell<DrawingBase>> {
        Rc::clone(&self.target)
    }
}

impl<T> Deref for BlendAnimator<T> {
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T> DerefMut for BlendAnimator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
