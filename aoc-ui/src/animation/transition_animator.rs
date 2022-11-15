use super::animator::*;
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct TransitionAnimator<
    T1: Animator + TargetedAnimator<V>,
    T2: Animator + TargetedAnimator<V>,
    V: Drawable,
> {
    pub base: AnimatorBase,
    pub a: T1,
    pub b: T2,
    pub target: Rc<RefCell<V>>,
    pub length_ms: f32,
    pub elapsed: f32,
}

impl<T1: Animator + TargetedAnimator<V>, T2: Animator + TargetedAnimator<V>, V: Drawable>
    TransitionAnimator<T1, T2, V>
{
    pub fn new(
        a: T1,
        b: T2,
        target: Rc<RefCell<V>>,
        length_ms: f32,
    ) -> TransitionAnimator<T1, T2, V> {
        let start_pos = target.borrow().base().pos.clone();

        a.get_target().borrow_mut().base_mut().visible = false;
        b.get_target().borrow_mut().base_mut().visible = false;

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

impl<
        T1: Animator + TargetedAnimator<V> + 'static,
        T2: Animator + TargetedAnimator<V> + 'static,
        V: Drawable + 'static,
    > Animator for TransitionAnimator<T1, T2, V>
{
    fn tick(&mut self, ctx: &BTerm) {
        self.elapsed += ctx.frame_time_ms;
        self.a.tick(ctx);
        self.b.tick(ctx);

        let a = self.a.get_target();
        let mut a = a.borrow_mut();
        let a = a.base_mut();
        let b = self.b.get_target();
        let mut b = b.borrow_mut();
        let b = b.base_mut();

        let mut target_ = self.target.borrow_mut();
        let target = target_.base_mut();
        let phase_a = self.elapsed.min(self.length_ms) / self.length_ms;
        let phase_b = 1.0 - phase_a;
        target.pos.x = a.pos.x * phase_a + b.pos.x * phase_b;
        target.pos.y = a.pos.y * phase_a + b.pos.y * phase_b;
        target.rotation = a.rotation * phase_a + b.rotation * phase_b;
        target.scale = a.scale * phase_a + b.scale * phase_b;
        target.transparency = a.transparency * phase_a + b.transparency * phase_b;
    }

    fn state(&self) -> AnimationState {
        match self.elapsed >= self.length_ms {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }

    fn into_animator(self) -> Box<dyn Animator> {
        Box::new(self)
    }
}

impl<T1: Animator + TargetedAnimator<V>, T2: Animator + TargetedAnimator<V>, V: Drawable> Deref
    for TransitionAnimator<T1, T2, V>
{
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T1: Animator + TargetedAnimator<V>, T2: Animator + TargetedAnimator<V>, V: Drawable> DerefMut
    for TransitionAnimator<T1, T2, V>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
