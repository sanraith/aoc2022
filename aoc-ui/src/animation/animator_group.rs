use super::animator::{AnimationState, Animator, AnimatorBase, TargetedAnimator};
use bracket_terminal::prelude::BTerm;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct AnimatorGroup<T> {
    pub base: AnimatorBase,
    pub animators: Vec<Box<dyn Animator>>,
    pub target: Rc<RefCell<T>>,
}

impl<T> AnimatorGroup<T> {
    pub fn new(target: Rc<RefCell<T>>, animators: Vec<Box<dyn Animator>>) -> AnimatorGroup<T> {
        AnimatorGroup {
            base: Default::default(),
            animators,
            target,
        }
    }
}

impl<T: 'static> Animator for AnimatorGroup<T> {
    fn tick(&mut self, ctx: &BTerm) {
        for animator in self.animators.iter_mut() {
            animator.tick(ctx);
        }
    }

    fn state(&self) -> AnimationState {
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
        state
    }

    fn into_animator(self) -> Box<dyn Animator> {
        Box::new(self)
    }
}

impl<T: 'static> TargetedAnimator<T> for AnimatorGroup<T> {
    fn get_target(&self) -> Rc<RefCell<T>> {
        Rc::clone(&self.target)
    }
}

impl<T> Deref for AnimatorGroup<T> {
    type Target = AnimatorBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T> DerefMut for AnimatorGroup<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
