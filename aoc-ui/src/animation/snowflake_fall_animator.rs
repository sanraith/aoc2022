use super::animator::{AnimationState, Animator};
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;

pub struct SnowflakeFallAnimator {
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
    pub total_elapsed: f32,
    pub seed: f32,
    pub state: AnimationState,
}

impl<T: Drawable> Animator<T> for SnowflakeFallAnimator {
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        self.total_elapsed += ctx.frame_time_ms;
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;

        let mut target = target.base_mut();
        target.pos.y += elapsed_seconds * self.vy;

        let sin = (self.seed + self.total_elapsed / 300.0 * self.v_sin_x).sin() * self.d_sin_x;
        target.pos.x += self.vx * elapsed_seconds + sin - self.last_sin;
        target.pos.x %= self.max_x;
        self.last_sin = sin;

        target.rotation += self.v_rot * elapsed_seconds;
        target.rotation %= 360.0;

        self.state = match target.pos.y >= self.max_y {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}

impl Default for SnowflakeFallAnimator {
    fn default() -> Self {
        Self {
            d_sin_x: Default::default(),
            v_sin_x: Default::default(),
            vy: Default::default(),
            max_x: Default::default(),
            max_y: Default::default(),

            vx: 0.0,
            v_rot: 0.0,
            last_sin: 0.0,

            total_elapsed: 0.0,
            seed: rand::random::<f32>(),
            state: AnimationState::Running,
        }
    }
}
