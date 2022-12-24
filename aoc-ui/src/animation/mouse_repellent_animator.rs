use super::animator::{AnimationState, Animator};
use crate::{
    config::Config,
    drawing::drawing_base::Drawable,
    util::{distance2d_pythagoras_f32, get_mouse_tile_pos},
};
use bracket_terminal::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub struct MouseRepellentAnimator {
    pub config: Rc<RefCell<Config>>,
}

impl<T: Drawable> Animator<T> for MouseRepellentAnimator {
    fn tick(&mut self, ctx: &BTerm, target: &mut T) {
        let elapsed_seconds = ctx.frame_time_ms / 1000.0;
        let mouse_active = match self.config.borrow().mouse {
            (x, y) if x > 0.0 && y > 0.0 => true,
            _ => false,
        };
        let influence = 7.5;
        let power = 20.0;
        let mp = get_mouse_tile_pos(&self.config.borrow());

        let mut target = target.base_mut();
        let d = distance2d_pythagoras_f32(&target.pos, &mp);
        let direction_x = if target.pos.x > mp.x as f32 { 1 } else { -1 } as f32;
        let direction_y = if target.pos.y > mp.y as f32 { 1 } else { -1 } as f32;
        if mouse_active && d < influence {
            target.pos.x += ((influence - d) / influence) * power * direction_x * elapsed_seconds;
            target.pos.y += ((influence - d) / influence) * power * direction_y * elapsed_seconds;
        }
    }

    fn state(&self) -> &AnimationState {
        &AnimationState::RunningForever
    }
}
