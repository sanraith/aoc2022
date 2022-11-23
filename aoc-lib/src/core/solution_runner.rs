use futures::channel::mpsc::{self, UnboundedSender};
use futures::{executor, stream, SinkExt, Stream, StreamExt};
use std::thread;

// pub struct ResultPack {
//     pub result: SolutionResult,
//     pub time: f32,
// }

#[derive()]
pub enum ProgressEnum {
    Progress(f32),
    // Result(ResultPack),
    Done,
}

pub trait SolutionRunner {
    fn solve() -> Box<dyn Stream<Item = ProgressEnum>>;
}
pub struct ThreadSolutionRunner {}
impl SolutionRunner for ThreadSolutionRunner {
    fn solve() -> Box<dyn Stream<Item = ProgressEnum>> {
        let (tx, rx) = mpsc::unbounded::<ProgressEnum>();
        thread::spawn(move || executor::block_on(worker_thread(tx)));

        let progress_stream = stream::unfold(rx, |mut rx| async move {
            let item = rx
                .next()
                .await
                .expect("sink should not be closed before sending 'Done'");
            match item {
                ProgressEnum::Progress(_) => Some((item, rx)),
                ProgressEnum::Done => None,
            }
        });

        Box::new(progress_stream)
    }
}

async fn worker_thread(mut tx: UnboundedSender<ProgressEnum>) {
    _ = tx.feed(ProgressEnum::Progress(0.0)).await;
    _ = tx.feed(ProgressEnum::Progress(1.0)).await;
    _ = tx.send(ProgressEnum::Done).await;
    tx.close_channel();
}
