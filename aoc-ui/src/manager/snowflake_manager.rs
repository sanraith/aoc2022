use crate::{
    animation::{
        animator::{AnimationState, Animator, AnimatorBase, Targeted, TargetedAnimator},
        animator_group::AnimatorGroup,
        mouse_repellent_animator::MouseRepellentAnimator,
        snowflake_fall_animator::SnowflakeFallAnimator,
    },
    config::Config,
    drawing::{
        drawing_base::{Drawable, DrawingBase},
        snowflake::Snowflake,
    },
};
use bracket_terminal::prelude::{BTerm, DrawBatch, PointF};
use rand::{distributions::Uniform, prelude::Distribution, Rng};
use std::{cell::RefCell, rc::Rc};

pub struct AnimatedItem<T> {
    pub item: Rc<RefCell<T>>,
    pub animators: Vec<Box<dyn TargetedAnimator<T>>>,
    pub keep_after_animations: bool,
}

pub struct SnowflakeManager {
    snowflakes: Vec<AnimatedItem<Snowflake>>,
    config: Rc<RefCell<Config>>,
}
impl SnowflakeManager {
    pub fn new(config: Rc<RefCell<Config>>) -> Self {
        SnowflakeManager {
            config,
            snowflakes: Default::default(),
            // text: Default::default(),
            // text_flakes: Default::default(),
            // text_flake_queue: Default::default(),
        }
    }
    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        self.snowflakes.retain(|flake| {
            flake.keep_after_animations
                || flake.animators.iter().any(|a| match a.state() {
                    AnimationState::Running => true,
                    _ => false,
                })
        });
        self.create_snowflakes();

        // let mut random_flake = self
        //     .snowflakes
        //     .swap_remove(rand::thread_rng().gen_range(0..self.snowflakes.len()));

        // let mut to_group = Vec::new();
        // while random_flake.animators.len() > 0 {
        //     // TODO get owned trait object instead of trait ref
        //     let x = random_flake.animators.pop().unwrap().as_animator();
        //     to_group.push(x);
        // }
        // let group = AnimatorGroup::new(random_flake.item, to_group);

        // for flake in self.snowflakes.iter_mut() {
        //     flake.animators.iter_mut().for_each(|a| a.tick(ctx));
        //     flake.item.borrow().draw(ctx, batch);
        // }
    }

    fn create_snowflakes(&mut self) {
        let Config { width, height, .. } = *self.config.borrow();
        let mut rng = rand::thread_rng();
        let width_die = Uniform::from(0.0..width as f32);
        let height_die = Uniform::from(0.0..1.0);
        let height_starter_die = Uniform::from(0.0..height as f32);
        let snowflakes_count = 400;
        let height_die = match self.snowflakes.len() {
            0 => height_starter_die, // distribute flakes vertically initially
            _ => height_die,         // spawn new ones at the top
        };
        for _ in 0..snowflakes_count - self.snowflakes.len() {
            self.create_snowflake(&mut rng, width_die, height_die, width, height);
        }
    }

    fn create_snowflake(
        &mut self,
        rng: &mut rand::rngs::ThreadRng,
        width_die: Uniform<f32>,
        height_die: Uniform<f32>,
        max_x: u32,
        max_y: u32,
    ) {
        let flake = Rc::from(RefCell::from(Snowflake {
            base: DrawingBase {
                pos: PointF {
                    x: width_die.sample(rng),
                    y: height_die.sample(rng),
                },
                scale: rng.gen_range(0.25..1.0),
                rotation: rng.gen_range(0.0..180.0),
                ..Default::default()
            },
        }));

        let fall_animator = SnowflakeFallAnimator {
            base: AnimatorBase {
                start_pos: flake.borrow().pos.clone(),
                ..Default::default()
            },
            target: Rc::clone(&flake),
            d_sin_x: rng.gen_range(0.1..1.0),
            v_sin_x: rng.gen_range(0.2..0.7),
            vx: rng.gen_range(-1.5..1.5),
            vy: rng.gen_range(2.0..8.0),
            v_rot: rng.gen_range(-180.0..180.0),
            max_x: max_x as f32,
            max_y: max_y as f32 + 1.0,
            ..Default::default()
        };

        let mouse_animator = MouseRepellentAnimator {
            base: Default::default(),
            target: Rc::clone(&flake),
            config: Rc::clone(&self.config),
        };

        self.snowflakes.push(AnimatedItem {
            item: flake,
            animators: vec![Box::from(fall_animator), Box::from(mouse_animator)],
            keep_after_animations: false,
        });
    }
}
