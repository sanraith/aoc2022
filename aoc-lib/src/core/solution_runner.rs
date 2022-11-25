use crate::solution::{Context, ProgressHandler, Solution};
use crate::util::{GenericResult, YearDay};
use crate::{inputs, solutions};
use futures::channel::mpsc::{self, UnboundedSender};
use futures::{executor, stream, SinkExt, Stream, StreamExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, SystemTime};

pub struct ResultPack<T> {
    pub part: Option<u8>,
    pub value: T,
    pub duration: Duration,
}

#[derive()]
pub enum SolveProgress {
    Error(String),
    Progress(ResultPack<f32>),
    SuccessResult(ResultPack<String>),
    ErrorResult(ResultPack<String>),
    Done(ResultPack<()>),
}

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
        thread::spawn(move || executor::block_on(run_solution(day, input, tx)));

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

async fn close(tx: &Rc<RefCell<UnboundedSender<SolveProgress>>>, start: SystemTime) {
    let mut tx = tx.borrow_mut();
    _ = tx
        .send(SolveProgress::Done(ResultPack {
            part: None,
            value: (),
            duration: start.elapsed().unwrap_or_default(),
        }))
        .await;
    tx.close_channel();
}

async fn send_and_close(
    tx: &Rc<RefCell<UnboundedSender<SolveProgress>>>,
    start: SystemTime,
    msg: SolveProgress,
) {
    _ = tx.borrow_mut().feed(msg).await;
    close(tx, start).await;
}

pub struct SendOnProgress {
    tx: Rc<RefCell<UnboundedSender<SolveProgress>>>,
    min_duration_between_updates: Duration,
    start: SystemTime,
    last_update: SystemTime,
}
impl SendOnProgress {
    pub fn new(tx: Rc<RefCell<UnboundedSender<SolveProgress>>>) -> SendOnProgress {
        SendOnProgress {
            tx,
            min_duration_between_updates: Duration::from_millis(0),
            start: SystemTime::now(),
            last_update: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn new_with_fps(
        fps: f32,
        tx: Rc<RefCell<UnboundedSender<SolveProgress>>>,
    ) -> SendOnProgress {
        let mut sop = SendOnProgress::new(tx);
        sop.min_duration_between_updates = Duration::from_millis((1000.0 / fps.min(0.001)) as u64);
        return sop;
    }
}
impl ProgressHandler for SendOnProgress {
    fn on_progress(&mut self, value: f32) {
        let elapsed_since_last_update = SystemTime::now()
            .duration_since(self.last_update)
            .unwrap_or_default();
        if elapsed_since_last_update >= self.min_duration_between_updates {
            // TODO check whether block_on works here correctly
            _ = executor::block_on(self.tx.borrow_mut().send(SolveProgress::Progress(
                ResultPack {
                    part: None,
                    value,
                    duration: self.start.elapsed().unwrap_or_default(),
                },
            )));
            self.last_update = SystemTime::now();
        }
    }
}

async fn solve_part(
    solution: &mut Box<dyn Solution>,
    part: u8,
    global_start: SystemTime,
    ctx: &Context,
    tx: &Rc<RefCell<UnboundedSender<SolveProgress>>>,
) -> GenericResult<String> {
    let start = SystemTime::now();
    let result = match part {
        1 => solution.part1(ctx),
        2 => solution.part2(ctx),
        _ => Err("Invalid part!".into()),
    };

    let duration = SystemTime::now().duration_since(start).unwrap_or_default();
    match &result {
        Ok(result) => {
            _ = tx
                .borrow_mut()
                .send(SolveProgress::SuccessResult(ResultPack {
                    part: Some(part),
                    value: result.to_owned(),
                    duration,
                }))
                .await
        }
        Err(err) => {
            send_and_close(
                tx,
                global_start,
                SolveProgress::ErrorResult(ResultPack {
                    part: Some(part),
                    value: err.to_string(),
                    duration,
                }),
            )
            .await
        }
    };

    result
}

pub async fn run_solution(day: YearDay, input: Input, tx: UnboundedSender<SolveProgress>) {
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
                )
                .await
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
            )
            .await;
        }
    };

    if let Err(err) = solution.init(&ctx) {
        return send_and_close(
            &tx,
            start,
            SolveProgress::Error(format!("Unable to init solution: {}", err).to_owned()),
        )
        .await;
    }
    if let Err(_) = solve_part(&mut solution, 1, start, &ctx, &tx).await {
        return;
    }
    if let Err(_) = solve_part(&mut solution, 2, start, &ctx, &tx).await {
        return;
    }

    close(&tx, start).await;
}
