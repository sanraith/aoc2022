use super::{flake_text_manager::FlakeCharLine, snowflake_manager::SnowflakeManager};
use crate::{
    animation::{ease::EaseType, typed_line_animation::TypedLineAnimation},
    config::Config,
    drawing::drawing_base::DrawingBase,
    util::distance2d_pythagoras_f32,
};
use aoc::{core::solution_runner::SolveProgress, util::fmt_duration_s};
use bracket_terminal::prelude::{BTerm, DrawBatch, Point, PointF};
use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[allow(dead_code)]
static ROW_COUNT: usize = 45;
static COLUMN_WIDTH: usize = 29;
static TIME_PER_CHAR: f32 = 20.0;
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
            snowflake_manager,
            draw_queue: VecDeque::new(),
            snowy_lines: Vec::new(),
            typed_lines: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.snowy_lines.clear();
        for (_, typed_line) in self.typed_lines.iter_mut() {
            // keep title
            typed_line[1] = None;
            typed_line[2] = None;
        }
    }

    pub fn tick(&mut self, ctx: &BTerm, batch: &mut DrawBatch) {
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
                l.tick(ctx, batch);
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
                        .or_insert_with(|| Vec::from_iter([None, None, None]))[part] = Some(l);
                } else {
                    self.draw_queue
                        .push_front(QueueItem::TypedLine((day, part, l)));
                }
            }
            None => (),
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
                    format!("D{: <2} {}", day.day, title),
                    day.day,
                    Some(0 as u8),
                    TITLE_COLOR,
                    TIME_PER_CHAR,
                );
                self.add_typed_line(
                    "├P1...".to_owned(),
                    day.day,
                    Some(1 as u8),
                    PART_COLOR,
                    TIME_PER_CHAR,
                );
                self.add_typed_line(
                    "└P2...".to_owned(),
                    day.day,
                    Some(2 as u8),
                    PART_COLOR,
                    TIME_PER_CHAR,
                );
            }
            SolveProgress::Error(_) => (),
            SolveProgress::Progress(pack) => {
                let line = format!(
                    "{}P{} {: >8}► {: >5.2}%",
                    if pack.part.unwrap() == 1 {
                        "├"
                    } else {
                        "└"
                    },
                    pack.part.unwrap(),
                    format!("({})", fmt_duration_s(&pack.duration)),
                    pack.value * 100.0
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
                    "{}P{} {: >8}:",
                    if pack.part.unwrap() == 1 {
                        "├"
                    } else {
                        "└"
                    },
                    pack.part.unwrap(),
                    format!("({})", fmt_duration_s(&pack.duration)),
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
                let x = base_pos.x + 14;

                let mut flake_line = FlakeCharLine::new(
                    PointF::new(x as f32, y as f32),
                    FLAKE_CHAR_MOVE_TIME,
                    FLAKE_CHAR_FADE_OUT_TIME,
                    FLAKE_CHAR_FADE_IN_TIME,
                    SOLUTION_COLOR,
                );
                pack.value.chars().for_each(|c| flake_line.add_char(c));
                self.draw_queue.push_back(QueueItem::SnowyLine(flake_line));
            }
            SolveProgress::ErrorResult(_) => (),
            SolveProgress::Done(_) => (),
        }
    }

    fn add_typed_line(
        &mut self,
        line: String,
        day: u32,
        part: Option<u8>,
        color: (u8, u8, u8, u8),
        speed: f32,
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
                len * speed,
                EaseType::Linear,
            ),
        )));
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
