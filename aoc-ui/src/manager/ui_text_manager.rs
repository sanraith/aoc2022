use super::{flake_text_manager::FlakeCharLine, snowflake_manager::SnowflakeManager};
use crate::{
    animation::{
        animated_item::AnimatedItem, animator::AnimationState, animator_group::AnimatorGroup,
        ease::EaseType, simple_animator::SimpleAnimator,
        snowflake_fall_animator::SnowflakeFallAnimator, transition_animator::TransitionAnimator,
        typed_line_animation::TypedLineAnimation,
    },
    config::Config,
    drawing::{
        balloon::Balloon,
        drawing_base::{Drawable, DrawingBase},
    },
    state::BG_COLOR,
    util::distance2d_pythagoras_f32,
};
use aoc::{core::solution_runner::SolveProgress, util::fmt_duration_wasm};
use bracket_terminal::prelude::*;
use itertools::Itertools;
use rand::Rng;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[allow(dead_code)]
static ROW_COUNT: usize = 45;
static COLUMN_WIDTH: usize = 29;
static TIME_PER_CHAR: f32 = 25.0;
// static TIME_PER_CHAR: f32 = 2.0;
static FLAKE_CHAR_MOVE_TIME: f32 = 2000.0;
// static FLAKE_CHAR_MOVE_TIME: f32 = 10.0;
static FLAKE_CHAR_FADE_OUT_TIME: f32 = 1000.0;
static FLAKE_CHAR_FADE_IN_TIME: f32 = 500.0;

// static PART_COLOR: (u8, u8, u8, u8) = (127, 189, 57, 255); // green
static PART_COLOR: (u8, u8, u8, u8) = (0, 153, 0, 255); // aoc green

static TITLE_COLOR: (u8, u8, u8, u8) = (230, 230, 230, 255);
// static TITLE_COLOR: (u8, u8, u8, u8) = (204, 204, 204, 255);
// static SOLUTION_COLOR: (u8, u8, u8, u8) = (255, 255, 102, 255);
static SOLUTION_COLOR: (u8, u8, u8, u8) = (255, 255, 200, 255);

enum QueueItem {
    Progress(SolveProgress),
    TypedLine((u32, usize, TypedLineAnimation)),
    SnowyLine(FlakeCharLine),
}

pub struct UiTextManager {
    top_left: Point,
    _config: Rc<RefCell<Config>>,
    snowy_lines: Vec<FlakeCharLine>,
    typed_lines: HashMap<u32, Vec<Option<TypedLineAnimation>>>,
    snowflake_manager: Rc<RefCell<SnowflakeManager>>,
    draw_queue: VecDeque<QueueItem>,
    balloons: Vec<AnimatedItem<Balloon>>,
    speed: f32,
}
impl UiTextManager {
    pub fn new(
        config: Rc<RefCell<Config>>,
        snowflake_manager: Rc<RefCell<SnowflakeManager>>,
        top_left: Point,
    ) -> Self {
        let manager = UiTextManager {
            _config: config,
            top_left,
            snowflake_manager,
            draw_queue: VecDeque::new(),
            snowy_lines: Vec::new(),
            typed_lines: HashMap::new(),
            balloons: Vec::new(),
            speed: 1.0,
        };
        manager
    }

    pub fn clear(&mut self) {
        self.snowy_lines.clear();
        for (_, typed_line) in self.typed_lines.iter_mut() {
            for i in 0..typed_line.len() {
                typed_line[i] = None;
            }
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch, fancy_batch: &mut DrawBatch) {
        for mut dyn_line in self.snowy_lines.iter_mut().collect_vec() {
            handle_snowy_line(
                &mut self.snowflake_manager.borrow_mut(),
                &mut dyn_line,
                ctx,
                batch,
            );
        }
        for typed_line in self
            .typed_lines
            .iter_mut()
            .flat_map(|(_, lines)| lines.iter_mut())
        {
            if let Some(l) = typed_line {
                l.tick(ctx, fancy_batch);
            }
        }

        match self.draw_queue.pop_front() {
            Some(QueueItem::Progress(p)) => self.handle_progress(p),
            Some(QueueItem::SnowyLine(mut l)) => {
                handle_snowy_line(&mut self.snowflake_manager.borrow_mut(), &mut l, ctx, batch);
                if l.progress() >= 0.7 {
                    self.snowy_lines.push(l);
                } else {
                    self.draw_queue.push_front(QueueItem::SnowyLine(l));
                }
            }
            Some(QueueItem::TypedLine((day, part, mut l))) => {
                if self
                    .typed_lines
                    .get(&day)
                    .and_then(|v| Some(v[part].is_some()))
                    .unwrap_or(false)
                {
                    self.typed_lines.get_mut(&day).unwrap()[part] = None;
                }

                l.tick(ctx, batch);
                if l.progress() >= 1.0 {
                    self.typed_lines
                        .entry(day)
                        .or_insert_with(|| Vec::from_iter((0..10).map(|_| None)))[part] = Some(l);
                } else {
                    self.draw_queue
                        .push_front(QueueItem::TypedLine((day, part, l)));
                }
            }
            None => (),
        }

        let mut balloons_active = false;
        if let Some(lines) = self.typed_lines.get(&26) {
            if let Some(line) = lines.get(9) {
                if line.is_some() && line.as_ref().unwrap().state() == AnimationState::Completed {
                    balloons_active = true;
                }
            }
        }
        if balloons_active {
            self.handle_balloons(ctx, fancy_batch);
        } else {
            self.balloons.clear();
        }
    }

    fn handle_balloons(&mut self, ctx: &BTerm, fancy_batch: &mut DrawBatch) {
        if self.balloons.len() == 0 || self.balloons.last().unwrap().item.base.pos.x > 72.0 {
            let mut rng = rand::thread_rng();
            let colors = [
                RED, ORANGE, YELLOW, GREEN, CYAN, MAGENTA, GOLD, LIME, CORAL, PINK,
            ]
            .into_iter()
            .map(|x| (x.0, x.1, x.2, 255))
            .collect_vec();
            let balloon = Balloon {
                base: DrawingBase {
                    pos: PointF::new(rng.gen_range(66.5..67.5), rng.gen_range(41.0..=41.0)),
                    scale: 0.20,
                    color: colors[rng.gen_range(0..colors.len()) as usize],
                    opaqueness: 0.0,
                    ..Default::default()
                },
            };
            let nop = SimpleAnimator::<Balloon, _>::new(|_, _| AnimationState::Completed);
            let faded_in_state = SimpleAnimator::<Balloon, _>::new(|_, target| {
                target.base.opaqueness = 1.0;
                AnimationState::Completed
            });
            let fall_animator = SnowflakeFallAnimator {
                d_sin_y: rng.gen_range(0.2..0.7),
                v_sin_y: rng.gen_range(0.2..0.5),
                vx: 1.0,
                vy: -0.1,
                v_rot: 0.0,
                max_x: 1000 as f32,
                max_y: 1000 as f32 + 1.0,
                ..Default::default()
            };
            let start_transition = TransitionAnimator::new(
                &balloon,
                500.0,
                EaseType::EaseInOutQuad,
                nop,
                AnimatorGroup::new(vec![Box::from(faded_in_state), Box::from(fall_animator)]),
            );

            let balloon_item = AnimatedItem::<Balloon> {
                item: balloon,
                animators: vec![Box::from(start_transition)],
            };
            self.balloons.push(balloon_item);
        }

        while let Some(pos) = self.balloons.iter().position(|b| b.item.base.pos.x > 100.0) {
            self.balloons.remove(pos);
        }

        for balloon in self.balloons.iter_mut() {
            balloon
                .animators
                .iter_mut()
                .for_each(|a| a.tick(ctx, &mut balloon.item));

            if let Some(pos) = balloon
                .animators
                .iter()
                .position(|a| *a.state() == AnimationState::Completed)
            {
                balloon.animators.remove(pos);
            }

            balloon.item.draw(ctx, fancy_batch);
        }
    }

    pub fn update_progress(&mut self, progress: SolveProgress) {
        if self.draw_queue.len() > 0 {
            self.draw_queue.push_back(QueueItem::Progress(progress));
            return;
        }

        self.handle_progress(progress);
    }

    fn handle_progress(&mut self, progress: SolveProgress) {
        match progress {
            SolveProgress::Start(day, title) => {
                self.add_typed_line(
                    format!("#{: <2} {}", day.day, title),
                    day.day,
                    Some(0 as u8),
                    TITLE_COLOR,
                    TIME_PER_CHAR,
                );
                self.add_typed_line(
                    "├Part 1...".to_owned(),
                    day.day,
                    Some(1 as u8),
                    PART_COLOR,
                    TIME_PER_CHAR,
                );
                self.add_typed_line(
                    "└Part 2...".to_owned(),
                    day.day,
                    Some(2 as u8),
                    PART_COLOR,
                    TIME_PER_CHAR,
                );
            }
            SolveProgress::Error(_) => (),
            SolveProgress::Progress(pack) => {
                let line = format!(
                    "{}Part {}► {: >5.2}% {: >8}",
                    if pack.part.unwrap() == 1 {
                        "├"
                    } else {
                        "└"
                    },
                    pack.part.unwrap(),
                    pack.value * 100.0,
                    format!("({})", fmt_duration_wasm(&pack.duration)),
                );
                self.add_typed_line(
                    line,
                    pack.year_day.day,
                    pack.part,
                    PART_COLOR,
                    TIME_PER_CHAR,
                );
            }
            SolveProgress::SuccessResult(pack) => {
                let line = format!(
                    "{}Part {}:",
                    if pack.part.unwrap() == 1 {
                        "├"
                    } else {
                        "└"
                    },
                    pack.part.unwrap(),
                    // format!("({})", fmt_duration_s(&pack.duration)),
                );
                self.add_typed_line(
                    line,
                    pack.year_day.day,
                    pack.part,
                    PART_COLOR,
                    TIME_PER_CHAR,
                );

                let y = (pack.year_day.day - 1) as usize * 5;
                let base_pos =
                    self.top_left + Point::new((y / ROW_COUNT) * COLUMN_WIDTH, y % ROW_COUNT);
                let y = base_pos.y + pack.part.unwrap() as i32;
                let x = base_pos.x + 9;

                let mut flake_line = FlakeCharLine::new(
                    PointF::new(x as f32, y as f32),
                    FLAKE_CHAR_MOVE_TIME * self.speed,
                    FLAKE_CHAR_FADE_OUT_TIME * self.speed,
                    FLAKE_CHAR_FADE_IN_TIME * self.speed,
                    SOLUTION_COLOR,
                );

                pack.value.chars().for_each(|c| flake_line.add_char(c));
                self.draw_queue.push_back(QueueItem::SnowyLine(flake_line));
            }
            SolveProgress::ErrorResult(_) => (),
            SolveProgress::Done(pack) => {
                self.add_typed_line(
                    format!("({})", fmt_duration_wasm(&pack.duration)),
                    pack.year_day.day,
                    Some(3),
                    (100, 100, 100, 255),
                    TIME_PER_CHAR,
                );

                if pack.year_day.day == 25 {
                    self.draw_mountain();
                }
            }
        }
    }

    fn add_typed_line(
        &mut self,
        line: String,
        day: u32,
        part: Option<u8>,
        color: (u8, u8, u8, u8),
        base_speed: f32,
    ) {
        let part = part.unwrap() as usize;
        let y = (day - 1) as usize * 5;
        let base_pos = self.top_left + Point::new((y / ROW_COUNT) * COLUMN_WIDTH, y % ROW_COUNT);
        let len = line.len() as f32;

        while let Some(index) = self
            .draw_queue
            .iter()
            .enumerate()
            .find(|(_, x)| match x {
                QueueItem::TypedLine((d, p, _)) => *d == day && *p == part,
                _ => false,
            })
            .map(|(i, _)| i)
        {
            self.draw_queue.remove(index);
        }

        self.draw_queue.push_back(QueueItem::TypedLine((
            day,
            part,
            TypedLineAnimation::new(
                DrawingBase {
                    pos: PointF::new(base_pos.x as f32, base_pos.y as f32 + part as f32),
                    color,
                    ..Default::default()
                },
                line,
                len * base_speed * self.speed,
                EaseType::Linear,
                (BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 50),
            ),
        )));
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn draw_mountain(&mut self) {
        let mountain = r"
           .-.                      
          /   \                     
      .--'\/\_ \                    
    _/ ^      \/\'__                
   /    .'   _/  /  \               
  / :' __  ^/  ^/    `--.           
 /\  _/  \-' __/.' ^ _   \_   .'\   
/  \/     \ / -.   _/ \ -. `_/   \/\";
        mountain
            .lines()
            .take(9)
            .chain([vec![' '; 36].into_iter().collect::<String>().as_str()])
            .enumerate()
            .for_each(|(y, line)| {
                let white = ((350 as f32 - y as f32 / 6.0 * 255.0) as u8)
                    .min(255)
                    .max(0);
                let line = line.chars().take(36).chain([' ']).collect::<String>();
                let len = line.len();
                let base_pos = Point::new(53, 40 + y);
                let color = (
                    white,
                    150 + (white as f32 * (105.0 / 255.0)) as u8,
                    white,
                    255,
                );
                let bg_color = (
                    BG_COLOR.0,
                    BG_COLOR.1,
                    BG_COLOR.2,
                    (y as f32 / 6.0 * 255.0).min(255.0) as u8,
                );
                self.draw_queue.push_back(QueueItem::TypedLine((
                    26,
                    y,
                    TypedLineAnimation::new(
                        DrawingBase {
                            pos: PointF::new(base_pos.x as f32, base_pos.y as f32),
                            color,
                            ..Default::default()
                        },
                        line,
                        len as f32 * TIME_PER_CHAR * self.speed,
                        EaseType::EaseInOutCubic,
                        bg_color,
                    ),
                )));
            });
    }
}

fn handle_snowy_line(
    flake_man: &mut SnowflakeManager,
    dyn_line: &mut FlakeCharLine,
    ctx: &BTerm,
    batch: &mut DrawBatch,
) {
    let required_flake_count = dyn_line.required_flakes();
    if required_flake_count > 0 {
        let max_flakes_to_remove = std::usize::MAX
            .min(flake_man.snowflakes.len())
            .min(required_flake_count as usize);

        let mut flakes = Vec::new();
        let target_pos = dyn_line.pos - PointF::new(0.0, 10.0);
        for _ in 0..max_flakes_to_remove {
            let closest_flake_idx = flake_man
                .snowflakes
                .iter()
                .enumerate()
                .find(|(_, s)| distance2d_pythagoras_f32(&s.item.pos, &target_pos) < 20.0)
                .map(|(i, _)| i)
                .unwrap_or(0);
            flakes.push(flake_man.snowflakes.swap_remove(closest_flake_idx));
        }
        dyn_line.add_flakes(flakes);
    }
    dyn_line.tick(ctx, batch);
}
