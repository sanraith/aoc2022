use bracket_terminal::prelude::{BTerm, PointF};
use std::{cell::RefCell, rc::Rc};

pub struct AnimatorBase {
    pub start_pos: PointF,
    pub total_elapsed: f32,
    pub seed: f32,
}
impl Default for AnimatorBase {
    fn default() -> Self {
        Self {
            start_pos: PointF { x: 0.0, y: 0.0 },
            total_elapsed: 0.0,
            seed: rand::random(),
        }
    }
}

pub trait Animator<T> {
    fn tick(&mut self, ctx: &BTerm);
    fn is_completed(&self) -> bool;
    fn get_target(&self) -> Rc<RefCell<T>>;
}
