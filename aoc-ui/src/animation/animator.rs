use crate::drawing::drawing_base::Drawable;
use bracket_terminal::prelude::BTerm;

pub trait Animator<T: Drawable> {
    fn tick(&mut self, ctx: &BTerm, target: &mut T);
    fn state(&self) -> &AnimationState;
}

#[derive(PartialEq, Eq, Clone)]
pub enum AnimationState {
    Completed,
    Running,
    RunningForever,
}
