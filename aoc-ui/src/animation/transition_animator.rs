use super::animator::*;
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;

pub struct TransitionAnimator<T, A, B>
where
    T: Drawable + Clone,
    A: Animator<T>,
    B: Animator<T>,
{
    pub anim_a: A,
    pub anim_b: B,
    pub target_a: T,
    pub target_b: T,
    pub length_ms: f32,
    pub elapsed: f32,
    pub state: AnimationState,
}
impl<T, A, B> TransitionAnimator<T, A, B>
where
    T: Drawable + Clone,
    A: Animator<T>,
    B: Animator<T>,
{
    pub fn new(target: &T, length_ms: f32, anim_a: A, anim_b: B) -> TransitionAnimator<T, A, B> {
        TransitionAnimator {
            anim_a,
            anim_b,
            target_a: target.clone(),
            target_b: target.clone(),
            length_ms,
            elapsed: 0.0,
            state: AnimationState::Running,
        }
    }
}

impl<T, A, B> Animator<T> for TransitionAnimator<T, A, B>
where
    T: Drawable + Clone,
    A: Animator<T>,
    B: Animator<T>,
{
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        let transition_done = self.elapsed >= self.length_ms;
        self.elapsed += ctx.frame_time_ms;

        if !transition_done {
            self.anim_a.tick(ctx, &mut self.target_a);
        }
        self.anim_b.tick(ctx, &mut self.target_b);
        let a = self.target_a.base();
        let b = self.target_b.base();

        let phase_b = (self.elapsed.min(self.length_ms) / self.length_ms).min(1.0);
        let phase_a = (1.0 - phase_b).max(0.0);
        let mut target = target.base_mut();
        target.pos.x = a.pos.x * phase_a + b.pos.x * phase_b;
        target.pos.y = a.pos.y * phase_a + b.pos.y * phase_b;
        target.rotation = a.rotation * phase_a + b.rotation * phase_b;
        target.scale = a.scale * phase_a + b.scale * phase_b;
        target.transparency = a.transparency * phase_a + b.transparency * phase_b;

        self.state = match transition_done {
            true => self.anim_b.state().clone(),
            false => AnimationState::Running,
        }
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
