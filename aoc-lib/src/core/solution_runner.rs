use crate::util::YearDay;
use futures::channel::mpsc::{self, UnboundedSender};
use futures::{executor, stream, SinkExt, Stream, StreamExt};
use std::thread;

pub struct ResultPack {
    pub value: String,
    pub time: f32,
}

#[derive()]
pub enum SolveProgress {
    Progress(f32),
    SuccessResult(ResultPack),
    ErrorResult(ResultPack),
    Done,
}

pub enum Input {
    Default,
    Custom(String),
}

pub trait SolutionRunner {
    fn solve(day: YearDay, input: Input) -> Box<dyn Stream<Item = SolveProgress>>;
}
pub struct ThreadSolutionRunner {}
impl SolutionRunner for ThreadSolutionRunner {
    fn solve(day: YearDay, input: Input) -> Box<dyn Stream<Item = SolveProgress>> {
        let (tx, rx) = mpsc::unbounded::<SolveProgress>();
        thread::spawn(move || executor::block_on(worker_thread(day, input, tx)));

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

async fn worker_thread(_day: YearDay, _input: Input, mut tx: UnboundedSender<SolveProgress>) {
    _ = tx.feed(SolveProgress::Progress(0.0)).await;
    _ = tx.feed(SolveProgress::Progress(1.0)).await;
    _ = tx.send(SolveProgress::Done).await;
    tx.close_channel();
}
