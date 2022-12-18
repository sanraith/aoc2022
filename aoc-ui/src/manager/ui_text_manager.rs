use super::{flake_text_manager::FlakeCharLine, snowflake_manager::SnowflakeManager};
use crate::config::Config;
use aoc::{core::solution_runner::SolveProgress, util::fmt_duration};
use bracket_terminal::prelude::{BTerm, DrawBatch, Point};
use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
static ROW_COUNT: usize = 40;
static COLUMN_WIDTH: usize = 30;

pub struct UiTextManager {
    top_left: Point,
    _config: Rc<RefCell<Config>>,
    static_lines: Vec<String>,
    dyn_lines: Vec<FlakeCharLine>,
    _snowflake_manager: Rc<RefCell<SnowflakeManager>>,
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
            _snowflake_manager: snowflake_manager,
        }
    }

    pub fn clear(&mut self) {
        self.static_lines.clear();
        self.dyn_lines.clear();
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
        for (y, line) in self.static_lines.iter().enumerate() {
            batch.print(
                self.top_left + Point::new((y / ROW_COUNT) * COLUMN_WIDTH, y % ROW_COUNT),
                line,
            );
        }
        for dyn_line in &mut self.dyn_lines {
            dyn_line.tick(ctx, batch);
        }
    }

    pub fn update_progress(&mut self, progress: SolveProgress) {
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
                    "{}P{} {: >9}► {: >5.2}%",
                    if x.part.unwrap() == 1 { "├" } else { "└" },
                    x.part.unwrap(),
                    format!("({})", fmt_duration(&x.duration)),
                    x.value * 100.0
                );
            }
            SolveProgress::SuccessResult(x) => {
                let len = self.static_lines.len();
                self.static_lines[len - 3 + x.part.unwrap() as usize] = format!(
                    "{}P{} {: >9}: {}",
                    if x.part.unwrap() == 1 { "├" } else { "└" },
                    x.part.unwrap(),
                    format!("({})", fmt_duration(&x.duration)),
                    x.value
                );
            }
            SolveProgress::ErrorResult(_) => (),
            SolveProgress::Done(_) => {
                self.static_lines.push(String::default());
                self.static_lines.push(String::default());
            }
        }
    }
}
