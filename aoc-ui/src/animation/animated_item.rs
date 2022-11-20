use super::animator::Animator;

pub struct AnimatedItem<T> {
    pub item: T,
    pub animators: Vec<Box<dyn Animator<T>>>,
}
