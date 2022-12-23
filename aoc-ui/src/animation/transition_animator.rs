use super::{animator::*, ease::*};
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
    pub ease: EaseType,
}
impl<T, A, B> TransitionAnimator<T, A, B>
where
    T: Drawable + Clone,
    A: Animator<T>,
    B: Animator<T>,
{
    pub fn new(
        target: &T,
        length_ms: f32,
        ease: EaseType,
        anim_a: A,
        anim_b: B,
    ) -> TransitionAnimator<T, A, B> {
        TransitionAnimator {
            anim_a,
            anim_b,
            target_a: target.clone(),
            target_b: target.clone(),
            length_ms,
            ease,
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
        let phase_b = (self.elapsed / self.length_ms).min(1.0);
        let transition_done = phase_b >= 1.0;
        let phase_b = ease_progress(phase_b, &self.ease);
        let phase_a = 1.0 - phase_b;

        if !transition_done {
            self.anim_a.tick(ctx, &mut self.target_a);
        }
        self.anim_b.tick(ctx, &mut self.target_b);
        let a = self.target_a.base();
        let b = self.target_b.base();

        let mut target = target.base_mut();
        target.pos.x = a.pos.x * phase_a + b.pos.x * phase_b;
        target.pos.y = a.pos.y * phase_a + b.pos.y * phase_b;
        target.rotation = a.rotation * phase_a + b.rotation * phase_b;
        target.scale = a.scale * phase_a + b.scale * phase_b;
        target.opaqueness = a.opaqueness * phase_a + b.opaqueness * phase_b;
        target.color.0 = (a.color.0 as f32 * phase_a + b.color.0 as f32 * phase_b) as u8;
        target.color.1 = (a.color.1 as f32 * phase_a + b.color.1 as f32 * phase_b) as u8;
        target.color.2 = (a.color.2 as f32 * phase_a + b.color.2 as f32 * phase_b) as u8;

        self.state = match transition_done {
            true => self.anim_b.state().clone(),
            false => AnimationState::Running,
        }
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
