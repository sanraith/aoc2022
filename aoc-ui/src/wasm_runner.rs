use crate::js_interop::{self, WorkerCommand, JS_BRIDGE};
use aoc::{
    core::solution_runner::{Input, SolutionRunner, SolveProgress, Tx},
    helpers::AsSome,
    util::YearDay,
};
use futures::{channel::mpsc, executor, SinkExt};
use futures::{stream, Stream, StreamExt};
use wasm_bindgen::JsValue;

pub struct WasmRunner {}
impl SolutionRunner for WasmRunner {
    fn run(&self, day: YearDay, input: Input) -> Box<dyn Stream<Item = SolveProgress>> {
        let (tx, rx) = mpsc::unbounded::<SolveProgress>();
        let mut js_bridge = js_interop::JS_BRIDGE.lock().unwrap();
        if let Some(tx) = &mut js_bridge.worker_tx {
            executor::block_on(tx.close()).unwrap();
        };
        js_bridge.worker_tx = Some(tx);

        let progress_stream = stream::unfold(rx, |mut rx| async move {
            let item = rx.next().await;
            match item {
                Some(item) => Some((item, rx)),
                None => None,
            }
        });

        // Send solve day command to js worker
        js_bridge
            .worker_wrapper
            .as_some()
            .post_message(&JsValue::from_str(
                &serde_json::to_string(&WorkerCommand::StartDay(day, input)).unwrap(),
            ))
            .unwrap();

        Box::new(progress_stream)
    }
}

pub struct WorkerPostMessageTx {}
impl Tx<SolveProgress> for WorkerPostMessageTx {
    fn send(&mut self, msg: SolveProgress) {
        JS_BRIDGE
            .lock()
            .unwrap()
            .worker_scope_wrapper
            .as_some()
            .post_message(&JsValue::from_str(&serde_json::to_string(&msg).unwrap()))
            .unwrap();
    }

    fn close(&mut self) {
        // Do nothing, do not need to close channel between main and web worker
    }
}
