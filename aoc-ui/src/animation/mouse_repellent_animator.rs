use super::animator::{AnimationState, Animator, AnimatorBase, Targeted};
use crate::{
    config::Config,
    drawing::drawing_base::Drawable,
    util::{distance2d_pythagoras_f32, get_mouse_tile_pos},
};
use bracket_terminal::prelude::*;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct MouseRepellentAnimator<T: Drawable> {
    pub base: AnimatorBase,
    pub target: Rc<RefCell<T>>,
    pub config: Rc<RefCell<Config>>,
}

impl<T: Drawable> Animator for MouseRepellentAnimator<T> {
    fn tick(&mut self, ctx: &BTerm) {
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;
        let mouse_active = match INPUT.lock().mouse_pixel_pos() {
            (x, y) if x > 0.0 && y > 0.0 => true,
            _ => false,
        };
        let influence = 10.0;
        let power = 20.0;
        let mp = get_mouse_tile_pos(&self.config.borrow());

        let mut target_ = self.target.borrow_mut();
        let mut target = target_.base_mut();
        let d = distance2d_pythagoras_f32(&target.pos, &mp);
        let direction_x = if target.pos.x > mp.x as f32 { 1 } else { -1 } as f32;
        let direction_y = if target.pos.y > mp.y as f32 { 1 } else { -1 } as f32;
        if mouse_active && d < influence {
            target.pos.x += ((influence - d) / influence) * power * direction_x * elapsed_seconds;
            target.pos.y += ((influence - d) / influence) * power * direction_y * elapsed_seconds;
        }
    }

    fn state(&self) -> AnimationState {
        AnimationState::RunningForever
    }
}

impl<T: Drawable> Targeted<T> for MouseRepellentAnimator<T> {
    fn get_target(&self) -> Rc<RefCell<T>> {
        Rc::clone(&self.target)
    }
}

impl<T: Drawable> Deref for MouseRepellentAnimator<T> {
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl<T: Drawable> DerefMut for MouseRepellentAnimator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
