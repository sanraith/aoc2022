use bracket_terminal::prelude::BTerm;

use crate::drawing::drawing_base::Drawable;

pub trait Animator<T: Drawable> {
    fn tick(&mut self, ctx: &BTerm, target: &mut T);
    fn state(&self) -> &AnimationState;
}

#[derive(PartialEq, Eq)]
pub enum AnimationState {
    Completed,
    Running,
    RunningForever,
}
