use super::animator::{Animator, AnimatorBase};
use crate::drawing::snowflake::Snowflake;
use bracket_terminal::prelude::BTerm;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct SnowflakeFallAnimator {
    pub base: AnimatorBase,
    pub target: Rc<RefCell<Snowflake>>,

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

impl Animator<Snowflake> for SnowflakeFallAnimator {
    fn tick(&mut self, ctx: &BTerm) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;

        let mut target = self.target.borrow_mut();
        target.pos.y += elapsed_seconds * self.vy;

        let sin = (self.seed + self.total_elapsed / 300.0 * self.v_sin_x).sin() * self.d_sin_x;
        target.pos.x += self.vx * elapsed_seconds + sin - self.last_sin;
        target.pos.x %= self.max_x;
        self.last_sin = sin;

        target.rotation += self.v_rot * elapsed_seconds;
        target.rotation %= 360.0;
    }

    fn is_completed(&self) -> bool {
        self.target.borrow().pos.y >= self.max_y
    }

    fn get_target(&self) -> Rc<RefCell<Snowflake>> {
        Rc::clone(&self.target)
    }
}

impl Default for SnowflakeFallAnimator {
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

impl Deref for SnowflakeFallAnimator {
    type Target = AnimatorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for SnowflakeFallAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
