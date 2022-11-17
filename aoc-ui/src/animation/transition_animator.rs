use super::animator::*;
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;

struct TransitionAnimator<T, A, B>
where
    T: Drawable + Clone,
    A: Animator<T>,
    B: Animator<T>,
{
    anim_a: A,
    anim_b: B,
    target_a: T,
    target_b: T,
    length_ms: f32,
    elapsed: f32,
    state: AnimationState,
}
impl<T, A, B> TransitionAnimator<T, A, B>
where
    T: Drawable + Clone,
    A: Animator<T>,
    B: Animator<T>,
{
    pub fn _new(target: &T, length_ms: f32, anim_a: A, anim_b: B) -> TransitionAnimator<T, A, B> {
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
        self.elapsed += ctx.frame_time_ms;
        self.anim_a.tick(ctx, &mut self.target_a);
        self.anim_b.tick(ctx, &mut self.target_b);
        let a = self.target_a.base();
        let b = self.target_b.base();

        let phase_a = self.elapsed.min(self.length_ms) / self.length_ms;
        let phase_b = 1.0 - phase_a;
        let mut target = target.base_mut();
        target.pos.x = a.pos.x * phase_a + b.pos.x * phase_b;
        target.pos.y = a.pos.y * phase_a + b.pos.y * phase_b;
        target.rotation = a.rotation * phase_a + b.rotation * phase_b;
        target.scale = a.scale * phase_a + b.scale * phase_b;
        target.transparency = a.transparency * phase_a + b.transparency * phase_b;

        self.state = match self.elapsed >= self.length_ms {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
