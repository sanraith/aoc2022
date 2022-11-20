use crate::{
    animation::{
        animated_item::AnimatedItem, animator::AnimationState, animator_group::AnimatorGroup,
        ease::EaseType, simple_animator::SimpleAnimator, transition_animator::TransitionAnimator,
    },
    char_image::{self, CHARACTER_IMAGES},
    drawing::{
        character::Character,
        drawing_base::{Drawable, DrawingBase},
        snowflake::Snowflake,
    },
};
use bracket_terminal::prelude::{BTerm, DrawBatch, PointF};

const MOVE_TRANSITION_MS: f32 = 1500.0;
const FADE_TRANSITION_MS: f32 = 500.0;
const TARGET_FLAKE_SCALE: f32 = 0.35;

// Flake position corrections for less "pop-in"
const CORRECTION_X: f32 = -0.4;
const CORRECTION_Y: f32 = 0.6;

#[derive(Eq, PartialEq)]
pub enum CharState {
    Assembling,
    DoneAssembling,
    Fading,
    Done,
}

pub struct FlakeCharacter {
    pub char: AnimatedItem<Character>,
    pub queue: Vec<PointF>,
    pub flakes: Vec<AnimatedItem<Snowflake>>,
    pub state: CharState,
}
impl FlakeCharacter {
    pub fn new(char: char, base_pos: PointF) -> FlakeCharacter {
        let char_image = CHARACTER_IMAGES
            .get(&char)
            .or_else(|| CHARACTER_IMAGES.get(&' '))
            .expect("character image available");
        let pixels = char_image.rows.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &pixel)| pixel > 127)
                .map(move |(x, _)| (x, y))
        });
        let queue = pixels
            .map(|(x, y)| PointF {
                x: base_pos.x + x as f32 / char_image::CHAR_WIDTH as f32 + CORRECTION_X,
                y: base_pos.y as f32 + y as f32 / char_image::CHAR_HEIGHT as f32 + CORRECTION_Y,
            })
            .collect::<Vec<_>>();

        FlakeCharacter {
            char: AnimatedItem {
                item: Character {
                    glyph: char,
                    base: DrawingBase {
                        pos: base_pos,
                        opaqueness: 0.0,
                        ..Default::default()
                    },
                },
                animators: Default::default(),
            },
            queue,
            flakes: Default::default(),
            state: CharState::Assembling,
        }
    }
}

#[derive(Default)]
pub struct FlakeCharLine {
    pub pos: PointF,
    pub text: Vec<FlakeCharacter>,
}
impl FlakeCharLine {
    pub fn new(pos: PointF) -> FlakeCharLine {
        FlakeCharLine {
            pos,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        for flake_char in self.text.iter_mut() {
            if flake_char.state == CharState::Done {
                continue;
            }

            // Handle state changes
            let is_waiting = is_waiting_for_new_flakes_or_animation(flake_char);
            flake_char.state = match flake_char.state {
                CharState::Assembling if !is_waiting => CharState::DoneAssembling,
                CharState::Assembling => CharState::Assembling,
                CharState::DoneAssembling => CharState::Fading,
                CharState::Fading if !is_waiting => CharState::Done,
                CharState::Fading => CharState::Fading,
                CharState::Done => CharState::Done,
            };

            match flake_char.state {
                CharState::Assembling => animate_flakes(ctx, flake_char),
                CharState::DoneAssembling => start_fading(flake_char),
                CharState::Fading => animate_fading(ctx, flake_char),
                CharState::Done => flake_char.flakes.clear(),
            }
        }

        for fc in self.text.iter() {
            fc.flakes.iter().for_each(|f| f.item.draw(ctx, batch));
            fc.char.item.draw(ctx, batch);
        }
    }

    pub fn add_char(&mut self, char: char) {
        let pos = PointF {
            x: self.pos.x + self.text.len() as f32,
            y: self.pos.y,
        };
        let flake_char = FlakeCharacter::new(char, pos);
        self.text.push(flake_char);
    }

    pub fn add_flakes(&mut self, mut flakes: Vec<AnimatedItem<Snowflake>>) {
        let assembling_flake_chars = self
            .text
            .iter_mut()
            .filter(|fc| fc.state == CharState::Assembling);
        for fc in assembling_flake_chars {
            while fc.queue.len() > 0 && flakes.len() > 0 {
                let mut flake = flakes.pop().unwrap();
                let pos = fc.queue.pop().unwrap();
                let group = AnimatorGroup::new(flake.animators.drain(..).collect::<Vec<_>>());
                let move_to = SimpleAnimator::<Snowflake, _>::new(move |_, target| {
                    target.pos = pos;
                    target.scale = TARGET_FLAKE_SCALE;
                    AnimationState::Completed
                });
                let transition = TransitionAnimator::new(
                    &flake.item,
                    MOVE_TRANSITION_MS,
                    EaseType::EaseInOutCubic,
                    group,
                    move_to,
                );
                flake.animators.push(Box::new(transition));
                fc.flakes.push(flake);
            }

            if flakes.len() == 0 {
                break;
            }
        }
    }

    pub fn state(&self) -> AnimationState {
        match self.text.iter().all(|c| c.state == CharState::Done) {
            true => AnimationState::Completed,
            false => AnimationState::Running,
        }
    }

    pub fn required_flakes(&self) -> usize {
        self.text
            .iter()
            .map(|c| {
                if c.state == CharState::Assembling {
                    c.queue.len()
                } else {
                    0
                }
            })
            .sum()
    }
}

fn is_waiting_for_new_flakes_or_animation(flake_char: &mut FlakeCharacter) -> bool {
    flake_char.queue.len() > 0
        || flake_char.flakes.iter().any(|f| {
            f.animators
                .iter()
                .any(|a| *a.state() == AnimationState::Running)
        })
        || flake_char
            .char
            .animators
            .iter()
            .any(|a| *a.state() == AnimationState::Running)
}

fn start_fading(flake_char: &mut FlakeCharacter) {
    // fade flakes
    for flake in flake_char.flakes.iter_mut() {
        let prev_animators_group =
            AnimatorGroup::new(flake.animators.drain(..).collect::<Vec<_>>());
        flake.animators.clear();

        let fade_out = SimpleAnimator::<Snowflake, _>::new(|_, target| {
            target.opaqueness = 0.0;
            AnimationState::Completed
        });
        let transition = TransitionAnimator::new(
            &flake.item,
            FADE_TRANSITION_MS,
            EaseType::EaseInOutCubic,
            prev_animators_group,
            fade_out,
        );
        flake.animators.push(Box::new(transition));
    }

    // fade glyph
    let nop = SimpleAnimator::<Character, _>::new(|_, _| AnimationState::Completed);
    let fade_in = SimpleAnimator::<Character, _>::new(|_, target| {
        target.base.opaqueness = 1.0;
        AnimationState::Completed
    });
    let transition = TransitionAnimator::new(
        &flake_char.char.item,
        FADE_TRANSITION_MS,
        EaseType::EaseInOutCubic,
        nop,
        fade_in,
    );
    flake_char.char.animators.push(Box::new(transition));
}

fn animate_fading(ctx: &BTerm, flake_char: &mut FlakeCharacter) {
    animate_flakes(ctx, flake_char);
    animate_character(ctx, flake_char);
}

fn animate_character(ctx: &BTerm, flake_char: &mut FlakeCharacter) {
    flake_char
        .char
        .animators
        .iter_mut()
        .for_each(|a| a.tick(ctx, &mut flake_char.char.item));
}

fn animate_flakes(ctx: &BTerm, flake_char: &mut FlakeCharacter) {
    for flake in flake_char.flakes.iter_mut() {
        flake
            .animators
            .iter_mut()
            .for_each(|a| a.tick(ctx, &mut flake.item))
    }
}
