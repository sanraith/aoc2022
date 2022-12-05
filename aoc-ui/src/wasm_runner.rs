use crate::js_interop::{self, WorkerCommand};
use aoc::{
    core::solution_runner::{Input, LocalSyncStream, SolutionRunner, SyncStream},
    helpers::AsSome,
    util::YearDay,
};
use std::sync::{Arc, Mutex};
use wasm_bindgen::JsValue;

pub struct WasmRunner {}
impl SolutionRunner<LocalSyncStream> for WasmRunner {
    fn run(&self, day: YearDay, input: Input) -> Arc<Mutex<LocalSyncStream>> {
        let stream = Arc::new(Mutex::new(LocalSyncStream::new()));
        let mut js_bridge = js_interop::JS_BRIDGE.lock().unwrap();

        if let Some(stream) = &mut js_bridge.worker_tx {
            stream.lock().unwrap().close();
        };
        js_bridge.worker_tx = Some(Arc::clone(&stream));

        // Send solve day command to js worker
        js_bridge
            .worker_wrapper
            .as_some()
            .post_message(&JsValue::from_str(
                &serde_json::to_string(&WorkerCommand::StartDay(day, input)).unwrap(),
            ))
            .unwrap();

        stream
    }
}
