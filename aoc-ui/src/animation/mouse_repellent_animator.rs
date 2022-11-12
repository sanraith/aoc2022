use super::animator::{Animator, AnimatorBase};
use crate::{
    config::Config,
    drawing::drawing_base::DrawingBase,
    util::{distance2d_pythagoras_f32, get_mouse_tile_pos},
};
use bracket_terminal::prelude::*;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct MouseRepellentAnimator {
    base: AnimatorBase,
    target: Rc<RefCell<DrawingBase>>,
    config: Rc<RefCell<Config>>,
}

impl Animator<DrawingBase> for MouseRepellentAnimator {
    fn tick(&mut self, ctx: &BTerm) {
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;
        let mouse_active = match INPUT.lock().mouse_pixel_pos() {
            (x, y) if x > 0.0 && y > 0.0 => true,
            _ => false,
        };
        let influence = 10.0;
        let power = 20.0;
        let mp = get_mouse_tile_pos(&self.config.borrow());

        let mut target = self.target.borrow_mut();
        let d = distance2d_pythagoras_f32(&target.pos, &mp);
        let direction_x = if target.pos.x > mp.x as f32 { 1 } else { -1 } as f32;
        let direction_y = if target.pos.y > mp.y as f32 { 1 } else { -1 } as f32;
        if mouse_active && d < influence {
            target.pos.x += ((influence - d) / influence) * power * direction_x * elapsed_seconds;
            target.pos.y += ((influence - d) / influence) * power * direction_y * elapsed_seconds;
        }
    }

    fn is_completed(&self) -> bool {
        false
    }

    fn get_target(&self) -> Rc<RefCell<DrawingBase>> {
        Rc::clone(&self.target)
    }
}

impl Deref for MouseRepellentAnimator {
    type Target = AnimatorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
impl DerefMut for MouseRepellentAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}
