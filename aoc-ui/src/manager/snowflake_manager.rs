use crate::{
    animation::{
        animator::{AnimationState, Animator},
        animator_group::AnimatorGroup,
        mouse_repellent_animator::MouseRepellentAnimator,
        move_to_animator::MoveToAnimator,
        snowflake_fall_animator::SnowflakeFallAnimator,
        transition_animator::TransitionAnimator,
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

const SNOWFLAKE_COUNT: usize = 400;

#[derive(PartialEq, Eq)]
pub enum SnowflakeKind {
    Free,
    Occupied,
}

pub struct AnimatedItem<T> {
    pub item: T,
    pub animators: Vec<Box<dyn Animator<T>>>,
    pub keep_after_animations: bool,
    pub kind: SnowflakeKind,
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

        // Test animator grouping
        let random_flake_index = rand::thread_rng().gen_range(0..self.snowflakes.len());
        if self.snowflakes.get(random_flake_index).unwrap().kind == SnowflakeKind::Free {
            let mut random_flake = self.snowflakes.swap_remove(random_flake_index);
            let mut to_group = Vec::new();
            while random_flake.animators.len() > 0 {
                to_group.push(random_flake.animators.pop().unwrap());
            }

            let group = AnimatorGroup::new(to_group);
            let move_anim = MoveToAnimator {
                end_pos: PointF::from((0.0, 0.0)),
                end_delta: 0.1,
                v: 10.0,
                total_elapsed: 0.0,
                state: AnimationState::Running,
            };

            random_flake.item = random_flake.item.clone();
            let transition_anim =
                TransitionAnimator::new(&random_flake.item, 500.0, group, move_anim);
            random_flake.animators.push(Box::new(transition_anim));
            random_flake.kind = SnowflakeKind::Occupied;
            self.snowflakes.push(random_flake);
        }

        // end test animator grouping

        for flake in self.snowflakes.iter_mut() {
            flake
                .animators
                .iter_mut()
                .for_each(|a| a.tick(ctx, &mut flake.item));
            flake.item.draw(ctx, batch);
        }
    }

    fn create_snowflakes(&mut self) {
        let Config { width, height, .. } = *self.config.borrow();
        let mut rng = rand::thread_rng();
        let width_die = Uniform::from(0.0..width as f32);
        let height_die = Uniform::from(0.0..1.0);
        let height_starter_die = Uniform::from(0.0..height as f32);
        let snowflakes_count = SNOWFLAKE_COUNT;
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
        let flake = Snowflake {
            base: DrawingBase {
                pos: PointF {
                    x: width_die.sample(rng),
                    y: height_die.sample(rng),
                },
                scale: rng.gen_range(0.25..1.0),
                rotation: rng.gen_range(0.0..180.0),
                ..Default::default()
            },
        };

        let fall_animator = SnowflakeFallAnimator {
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
            config: Rc::clone(&self.config),
        };

        self.snowflakes.push(AnimatedItem {
            item: flake,
            animators: vec![Box::from(fall_animator), Box::from(mouse_animator)],
            keep_after_animations: false,
            kind: SnowflakeKind::Free,
        });
    }
}
