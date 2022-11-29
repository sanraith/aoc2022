use crate::solution::{Context, ProgressHandler, Solution};
use crate::util::{GenericResult, YearDay};
use crate::{inputs, solutions};
use futures::channel::mpsc::{self, UnboundedSender};
use futures::{executor, stream, SinkExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, SystemTime};

pub trait Tx<T> {
    fn send(&mut self, msg: T);
    fn close(&mut self);
}
struct UnboundedTxWrapper {
    tx: UnboundedSender<SolveProgress>,
}
impl Tx<SolveProgress> for UnboundedTxWrapper {
    fn send(&mut self, msg: SolveProgress) {
        executor::block_on(self.tx.send(msg)).unwrap(); // TODO check
    }

    fn close(&mut self) {
        self.tx.close_channel();
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResultPack<T> {
    pub part: Option<u8>,
    pub value: T,
    pub duration: Duration,
}

#[derive(Serialize, Deserialize)]
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

pub trait SolutionRunner {
    fn run(day: YearDay, input: Input) -> Box<dyn Stream<Item = SolveProgress>>;
}
pub struct ThreadSolutionRunner {}
impl SolutionRunner for ThreadSolutionRunner {
    fn run(day: YearDay, input: Input) -> Box<dyn Stream<Item = SolveProgress>> {
        let (tx, rx) = mpsc::unbounded::<SolveProgress>();
        thread::spawn(move || {
            executor::block_on(run_solution(day, input, UnboundedTxWrapper { tx }))
        });

        let progress_stream = stream::unfold(rx, |mut rx| async move {
            let item = rx.next().await;
            match item {
                Some(item) => Some((item, rx)),
                None => None,
            }
        });

        Box::new(progress_stream)
    }
}

pub async fn run_solution<T: Tx<SolveProgress> + 'static>(day: YearDay, input: Input, tx: T) {
    let tx = Rc::new(RefCell::new(tx));
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
    let ctx = Context {
        raw_input,
        progress_handler: Box::new(SendOnProgress::new_with_fps(20.0, Rc::clone(&tx))),
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

    if let Err(err) = solution.init(&ctx) {
        return send_and_close(
            &tx,
            start,
            SolveProgress::Error(format!("Unable to init solution: {}", err).to_owned()),
        );
    }
    if let Err(_) = solve_part(&mut solution, 1, start, &ctx, &tx).await {
        return;
    }
    if let Err(_) = solve_part(&mut solution, 2, start, &ctx, &tx).await {
        return;
    }

    close(&tx, start).await;
}

fn send_and_close<T: Tx<SolveProgress>>(
    tx: &Rc<RefCell<T>>,
    start: SystemTime,
    msg: SolveProgress,
) {
    tx.borrow_mut().send(msg); // TODO consider feed instead
    executor::block_on(close(tx, start));
}

async fn close<T: Tx<SolveProgress>>(tx: &Rc<RefCell<T>>, start: SystemTime) {
    let mut tx = tx.borrow_mut();
    tx.send(SolveProgress::Done(ResultPack {
        part: None,
        value: (),
        duration: start.elapsed().unwrap_or_default(),
    }));
    tx.close();
}

async fn solve_part<T: Tx<SolveProgress>>(
    solution: &mut Box<dyn Solution>,
    part: u8,
    global_start: SystemTime,
    ctx: &Context,
    tx: &Rc<RefCell<T>>,
) -> GenericResult<String> {
    let start = SystemTime::now();
    let result = match part {
        1 => solution.part1(ctx),
        2 => solution.part2(ctx),
        _ => Err("Invalid part!".into()),
    };

    let duration = SystemTime::now().duration_since(start).unwrap_or_default();
    match &result {
        Ok(result) => tx
            .borrow_mut()
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

pub struct SendOnProgress<T: Tx<SolveProgress>> {
    tx: Rc<RefCell<T>>,
    min_duration_between_updates: Duration,
    start: SystemTime,
    last_update: SystemTime,
}
impl<T: Tx<SolveProgress>> SendOnProgress<T> {
    pub fn new(tx: Rc<RefCell<T>>) -> SendOnProgress<T> {
        SendOnProgress {
            tx,
            min_duration_between_updates: Duration::from_millis(0),
            start: SystemTime::now(),
            last_update: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn new_with_fps(fps: f32, tx: Rc<RefCell<T>>) -> SendOnProgress<T> {
        let mut sop = SendOnProgress::new(tx);
        sop.min_duration_between_updates = Duration::from_millis((1000.0 / fps.min(0.001)) as u64);
        return sop;
    }
}
impl<T: Tx<SolveProgress>> ProgressHandler for SendOnProgress<T> {
    fn on_progress(&mut self, value: f32) {
        let elapsed_since_last_update = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap_or_default();
        if elapsed_since_last_update >= self.min_duration_between_updates {
            // TODO check whether block_on works here correctly
            self.tx
                .borrow_mut()
                .send(SolveProgress::Progress(ResultPack {
                    part: None,
                    value,
                    duration: self.start.elapsed().unwrap_or_default(),
                }));
            self.last_update = SystemTime::now();
        }
    }
}
