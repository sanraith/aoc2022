use super::animator::{AnimationState, Animator};
use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;
use std::marker::PhantomData;

pub struct SimpleAnimator<T, F>
where
    T: Drawable,
    F: Fn(&BTerm, &mut T) -> AnimationState,
{
    updater: F,
    state: AnimationState,
    _phantom: PhantomData<T>,
}

impl<T, F> SimpleAnimator<T, F>
where
    T: Drawable,
    F: Fn(&BTerm, &mut T) -> AnimationState,
{
    pub fn new(updater: F) -> SimpleAnimator<T, F> {
        SimpleAnimator {
            updater,
            state: AnimationState::RunningForever,
            _phantom: PhantomData,
        }
    }
}

impl<T, F> Animator<T> for SimpleAnimator<T, F>
where
    T: Drawable,
    F: Fn(&BTerm, &mut T) -> AnimationState,
{
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        self.state = (self.updater)(ctx, target);
    }

    fn state(&self) -> &AnimationState {
        &self.state
    }
}
