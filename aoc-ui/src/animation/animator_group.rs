use super::animator::{AnimationState, Animator};
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;

pub struct AnimatorGroup<T> {
    pub animators: Vec<Box<dyn Animator<T>>>,
    pub state: AnimationState,
}

impl<T> AnimatorGroup<T> {
    pub fn new(animators: Vec<Box<dyn Animator<T>>>) -> AnimatorGroup<T> {
        AnimatorGroup {
            animators,
            state: AnimationState::Running,
        }
    }
}

impl<T: Drawable> Animator<T> for AnimatorGroup<T> {
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        for animator in self.animators.iter_mut() {
            animator.tick(ctx, target);
        }

        let mut state = AnimationState::Completed;
        for animator in &self.animators {
            match animator.state() {
                AnimationState::Running => {
                    state = AnimationState::Running;
                    break;
                }
                AnimationState::RunningForever => state = AnimationState::RunningForever,
                AnimationState::Completed => (),
            }
        }
        self.state = state;
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
