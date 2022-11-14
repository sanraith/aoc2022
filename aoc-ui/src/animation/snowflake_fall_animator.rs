use super::animator::{AnimationState, Animator, AnimatorBase};
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct SnowflakeFallAnimator<T: Drawable> {
    pub base: AnimatorBase,
    pub target: Rc<RefCell<T>>,

    pub d_sin_x: f32,
    pub v_sin_x: f32,
    /** delta x / second */
    pub vx: f32,
    /** delta y / second */
    pub vy: f32,
    /** delta rotation degree / second */
    pub v_rot: f32,
    pub max_x: f32,
    pub max_y: f32,

    pub last_sin: f32,
}

impl<T: Drawable> Animator for SnowflakeFallAnimator<T> {
    fn tick(&mut self, ctx: &BTerm) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;

        let mut target_ = self.target.borrow_mut();
        let mut target = target_.base_mut();
        target.pos.y += elapsed_seconds * self.vy;

        let sin = (self.seed + self.total_elapsed / 300.0 * self.v_sin_x).sin() * self.d_sin_x;
        target.pos.x += self.vx * elapsed_seconds + sin - self.last_sin;
        target.pos.x %= self.max_x;
        self.last_sin = sin;

        target.rotation += self.v_rot * elapsed_seconds;
        target.rotation %= 360.0;
    }

    fn state(&self) -> AnimationState {
        match self.target.borrow().base().pos.y >= self.max_y {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }
}

impl<T: Drawable + Default> Default for SnowflakeFallAnimator<T> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            target: Default::default(),
            d_sin_x: Default::default(),
            v_sin_x: Default::default(),
            vy: Default::default(),
            max_x: Default::default(),
            max_y: Default::default(),

            vx: 0.0,
            v_rot: 0.0,
            last_sin: 0.0,
        }
    }
}

impl<T: Drawable> Deref for SnowflakeFallAnimator<T> {
    type Target = AnimatorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T: Drawable> DerefMut for SnowflakeFallAnimator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
