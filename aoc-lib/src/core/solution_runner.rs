use crate::solution::{Context, ProgressHandler, Solution};
use crate::util::{GenericResult, YearDay};
use crate::{inputs, solutions};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use wasm_timer::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultPack<T> {
    pub part: Option<u8>,
    pub value: T,
    pub duration: Duration,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SolveProgress {
    Error(String),
    Progress(ResultPack<f32>),
    SuccessResult(ResultPack<String>),
    ErrorResult(ResultPack<String>),
    Done(ResultPack<()>),
}

#[derive(Serialize, Deserialize)]
pub enum Input {
    Default,
    Custom(String),
}

pub trait SyncStream {
    fn send(&mut self, item: SolveProgress);
    fn close(&mut self);
    fn next_items(&mut self) -> Option<Vec<SolveProgress>>;
}

pub struct LocalSyncStream {
    items: Vec<SolveProgress>,
    active: bool,
}
impl LocalSyncStream {
    pub fn new() -> LocalSyncStream {
        LocalSyncStream {
            items: Default::default(),
            active: true,
        }
    }
}
impl SyncStream for LocalSyncStream {
    fn send(&mut self, item: SolveProgress) {
        self.items.push(item);
    }

    fn close(&mut self) {
        self.active = false;
    }

    /// Returns the next items, or None if the stream is closed.
    fn next_items(&mut self) -> Option<Vec<SolveProgress>> {
        match self.items.len() > 0 || self.active {
            true => Some(self.items.drain(..).collect_vec()),
            false => None,
        }
    }
}

pub trait SolutionRunner<T: SyncStream> {
    fn run(&self, day: YearDay, input: Input) -> Arc<Mutex<T>>;
}
pub struct ThreadSolutionRunner {}
impl SolutionRunner<LocalSyncStream> for ThreadSolutionRunner {
    fn run(&self, day: YearDay, input: Input) -> Arc<Mutex<LocalSyncStream>> {
        let stream = Arc::new(Mutex::new(LocalSyncStream::new()));
        let stream_copy = Arc::clone(&stream);
        thread::spawn(move || run_solution(day, input, stream_copy));

        stream
    }
}

pub fn run_solution<T: SyncStream + 'static>(day: YearDay, input: Input, tx: Arc<Mutex<T>>) {
    let start = SystemTime::now();
    let raw_input = match input {
        Input::Default => match inputs::get(&day) {
            Some(input) => input.to_owned(),
            None => {
                return send_and_close(
                    &tx,
                    start,
                    SolveProgress::Error("input not found".to_owned()),
                );
            }
        },
        Input::Custom(input) => input,
    };
    let current_part = Rc::new(RefCell::new(0));
    let ctx = Context {
        raw_input,
        progress_handler: RefCell::new(Box::new(SendOnProgress::new_with_fps(
            20.1,
            Arc::clone(&tx),
            Rc::clone(&current_part),
        ))),
    };
    let mut solution = match solutions::create_map().get(&day) {
        Some(solutions) => solutions[0].create_new(),
        None => {
            return send_and_close(
                &tx,
                start,
                SolveProgress::Error("solution not found".to_owned()),
            );
        }
    };

    let start = SystemTime::now();
    if let Err(err) = solution.init(&ctx) {
        return send_and_close(
            &tx,
            start,
            SolveProgress::Error(format!("Unable to initialize solution: {}", err).to_owned()),
        );
    }
    if let Err(_) = solve_part(&mut solution, 1, start, &ctx, &tx, &current_part) {
        return;
    }
    if let Err(_) = solve_part(&mut solution, 2, start, &ctx, &tx, &current_part) {
        return;
    }

    close(&tx, start);
}

fn send_and_close<T: SyncStream>(tx: &Arc<Mutex<T>>, start: SystemTime, msg: SolveProgress) {
    tx.lock().unwrap().send(msg);
    close(tx, start);
}

fn close<T: SyncStream>(tx: &Arc<Mutex<T>>, start: SystemTime) {
    let mut tx = tx.lock().unwrap();
    tx.send(SolveProgress::Done(ResultPack {
        part: None,
        value: (),
        duration: start.elapsed().unwrap_or_default(),
    }));
    tx.close();
}

fn solve_part<T: SyncStream>(
    solution: &mut Box<dyn Solution>,
    part: u8,
    global_start: SystemTime,
    ctx: &Context,
    tx: &Arc<Mutex<T>>,
    current_part: &Rc<RefCell<u8>>,
) -> GenericResult<String> {
    let start = SystemTime::now();
    *current_part.borrow_mut() = part;
    let result = match part {
        1 => solution.part1(ctx),
        2 => solution.part2(ctx),
        _ => Err("Invalid part!".into()),
    };

    let duration = SystemTime::now().duration_since(start).unwrap_or_default();
    match &result {
        Ok(result) => tx
            .lock()
            .unwrap()
            .send(SolveProgress::SuccessResult(ResultPack {
                part: Some(part),
                value: result.to_owned(),
                duration,
            })),
        Err(err) => {
            send_and_close(
                &tx,
                global_start,
                SolveProgress::ErrorResult(ResultPack {
                    part: Some(part),
                    value: err.to_string(),
                    duration,
                }),
            );
        }
    };

    result
}

pub struct SendOnProgress<T: SyncStream> {
    tx: Arc<Mutex<T>>,
    min_duration_between_updates: Duration,
    start: SystemTime,
    last_update: SystemTime,
    current_part: Rc<RefCell<u8>>,
}
impl<T: SyncStream> SendOnProgress<T> {
    pub fn new(tx: Arc<Mutex<T>>, current_part: Rc<RefCell<u8>>) -> SendOnProgress<T> {
        SendOnProgress {
            tx,
            min_duration_between_updates: Duration::from_millis(0),
            start: SystemTime::now(),
            last_update: SystemTime::UNIX_EPOCH,
            current_part,
        }
    }

    pub fn new_with_fps(
        fps: f32,
        tx: Arc<Mutex<T>>,
        current_part: Rc<RefCell<u8>>,
    ) -> SendOnProgress<T> {
        let mut sop = SendOnProgress::new(tx, current_part);
        sop.min_duration_between_updates = Duration::from_millis((1000.0 / fps.max(0.001)) as u64);
        return sop;
    }
}
impl<T: SyncStream> ProgressHandler for SendOnProgress<T> {
    fn on_progress(&mut self, value: f32) {
        let elapsed_since_last_update = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap_or_default();
        if elapsed_since_last_update >= self.min_duration_between_updates {
            self.tx
                .lock()
                .unwrap()
                .send(SolveProgress::Progress(ResultPack {
                    part: Some(*self.current_part.borrow()),
                    value,
                    duration: self.start.elapsed().unwrap_or_default(),
                }));
            self.last_update = SystemTime::now();
        }
    }
}
