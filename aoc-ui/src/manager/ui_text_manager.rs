use super::{flake_text_manager::FlakeCharLine, snowflake_manager::SnowflakeManager};
use crate::{animation::animator::AnimationState, config::Config, util::distance2d_pythagoras_f32};
use aoc::{core::solution_runner::SolveProgress, helpers::AsSome, util::fmt_duration_s};
use bracket_terminal::prelude::{BTerm, DrawBatch, Point, PointF};
use itertools::Itertools;
use rand::Rng;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[allow(dead_code)]
static ROW_COUNT: usize = 40;
static COLUMN_WIDTH: usize = 29;

pub struct UiTextManager {
    top_left: Point,
    _config: Rc<RefCell<Config>>,
    static_lines: Vec<String>,
    dyn_lines: Vec<FlakeCharLine>,
    snowflake_manager: Rc<RefCell<SnowflakeManager>>,
    progress_queue: VecDeque<SolveProgress>,
}
impl UiTextManager {
    pub fn new(
        config: Rc<RefCell<Config>>,
        snowflake_manager: Rc<RefCell<SnowflakeManager>>,
        top_left: Point,
    ) -> Self {
        UiTextManager {
            _config: config,
            top_left,
            static_lines: Vec::new(),
            dyn_lines: Vec::new(),
            snowflake_manager,
            progress_queue: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        self.static_lines.clear();
        self.dyn_lines.clear();
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        while self.progress_queue.len() > 0
            && self
                .dyn_lines
                .last()
                .and_then(|x| Some(x.progress() >= 0.7))
                .unwrap_or(false)
        {
            let progress = self.progress_queue.pop_front().unwrap();
            self.update_progress(progress);
        }

        for (y, line) in self.static_lines.iter().enumerate() {
            let color = if y % 5 == 0 {
                bracket_terminal::prelude::ColorPair::new((200, 255, 255, 255), (0, 0, 0, 0))
            } else {
                bracket_terminal::prelude::ColorPair::new((200, 255, 200, 255), (0, 0, 0, 0))
            };
            batch.print_color(
                self.top_left + Point::new((y / ROW_COUNT) * COLUMN_WIDTH, y % ROW_COUNT),
                line,
                color,
            );
        }
        for dyn_line in &mut self.dyn_lines {
            let required_flake_count = dyn_line.required_flakes();
            let mut flake_man = self.snowflake_manager.borrow_mut();
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
                    // // .sorted_by(|(_, s1), (_, s2)| {
                    // //     distance2d_pythagoras_f32(&s1.item.pos, &dyn_line.pos)
                    // //         .total_cmp(&distance2d_pythagoras_f32(&s2.item.pos, &dyn_line.pos))
                    // // })
                    // .next()
                    // .unwrap()
                    // .0;

                    // let random_flake_index =
                    //     rand::thread_rng().gen_range(0..flake_man.snowflakes.len());
                    // flakes.push(flake_man.snowflakes.swap_remove(random_flake_index));
                    flakes.push(flake_man.snowflakes.swap_remove(closest_flake_idx));
                }
                dyn_line.add_flakes(flakes);
            }

            dyn_line.tick(ctx, batch);
        }
    }

    pub fn update_progress(&mut self, progress: SolveProgress) {
        if let Some(dyn_line) = self.dyn_lines.last() {
            match dyn_line.state() {
                AnimationState::Running if dyn_line.progress() < 0.7 => {
                    self.progress_queue.push_back(progress);
                    return;
                }
                _ => (),
            }
        }

        match progress {
            SolveProgress::Start(day, title) => {
                self.static_lines
                    .push(format!("D{: <2} {}", day.day, title));
                self.static_lines.push("├P1...".to_owned());
                self.static_lines.push("└P2...".to_owned());
            }
            SolveProgress::Error(_) => (),
            SolveProgress::Progress(x) => {
                let len = self.static_lines.len();
                self.static_lines[len - 3 + x.part.unwrap() as usize] = format!(
                    "{}P{} {: >8}► {: >5.2}%",
                    if x.part.unwrap() == 1 { "├" } else { "└" },
                    x.part.unwrap(),
                    format!("({})", fmt_duration_s(&x.duration)),
                    x.value * 100.0
                );
            }
            SolveProgress::SuccessResult(pack) => {
                let len = self.static_lines.len();

                self.static_lines[len - 3 + pack.part.unwrap() as usize] = format!(
                    "{}P{} {: >8}: {}",
                    if pack.part.unwrap() == 1 {
                        "├"
                    } else {
                        "└"
                    },
                    pack.part.unwrap(),
                    format!("({})", fmt_duration_s(&pack.duration)),
                    "" //x.value
                );

                let y = self.top_left.y
                    + (len as i32 - 3 + pack.part.unwrap() as i32) % ROW_COUNT as i32;
                let x = 14
                    + self.top_left.x
                    + (len as i32 - 3 + pack.part.unwrap() as i32) / ROW_COUNT as i32
                        * COLUMN_WIDTH as i32;

                let mut flake_line = FlakeCharLine::new(PointF::new(x as f32, y as f32));
                pack.value.chars().for_each(|c| flake_line.add_char(c));
                self.dyn_lines.push(flake_line);
            }
            SolveProgress::ErrorResult(_) => (),
            SolveProgress::Done(_) => {
                self.static_lines.push(String::default());
                self.static_lines.push(String::default());
            }
        }
    }
}
