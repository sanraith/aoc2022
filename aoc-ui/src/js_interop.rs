use crate::{entry::main, wasm_runner::WorkerPostMessageTx};
use aoc::{
    core::solution_runner::{self, Input, SolveProgress},
    helpers::AsSome,
    util::YearDay,
};
use futures::{channel::mpsc::UnboundedSender, executor, SinkExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    borrow::{Borrow, BorrowMut},
    ops::Deref,
    sync::Mutex,
};
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, Worker};

pub struct WorkerWrapper(Worker);
unsafe impl Send for WorkerWrapper {}
impl Deref for WorkerWrapper {
    type Target = Worker;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct WorkerScopeWrapper(DedicatedWorkerGlobalScope);
unsafe impl Send for WorkerScopeWrapper {}
impl Deref for WorkerScopeWrapper {
    type Target = DedicatedWorkerGlobalScope;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
pub enum WorkerCommand {
    StartDay(YearDay, Input),
}

#[derive(Default)]
pub struct JsBridge {
    pub scale: f64,
    pub unhandled_keys: Vec<String>,
    pub worker_tx: Option<UnboundedSender<SolveProgress>>,
    pub worker_wrapper: Option<WorkerWrapper>,
    pub worker_scope_wrapper: Option<WorkerScopeWrapper>,
}
pub static JS_BRIDGE: Lazy<Mutex<JsBridge>> = Lazy::new(|| Mutex::new(Default::default()));

// --- Worker ---

#[wasm_bindgen]
pub fn worker_set_global_scope(worker_scope: JsValue) {
    JS_BRIDGE.lock().unwrap().borrow_mut().worker_scope_wrapper = Some(WorkerScopeWrapper(
        DedicatedWorkerGlobalScope::from(worker_scope),
    ));
}

#[wasm_bindgen]
pub fn worker_on_message(message: JsValue) {
    let message = message.as_string().unwrap();
    let command: WorkerCommand = serde_json::from_str(&message).unwrap();

    match command {
        WorkerCommand::StartDay(year_day, _input) => {
            let temp_feedback = format!("{}/{}", year_day.year, year_day.day);
            JS_BRIDGE
                .lock()
                .unwrap()
                .borrow()
                .worker_scope_wrapper
                .as_some()
                .post_message(&JsValue::from(temp_feedback))
                .unwrap();

            let tx = WorkerPostMessageTx {};
            executor::block_on(solution_runner::run_solution(year_day, Input::Default, tx));
        }
    };
}

// --- Main ---

#[wasm_bindgen]
pub fn set_worker(worker: JsValue) {
    JS_BRIDGE.lock().unwrap().borrow_mut().worker_wrapper =
        Some(WorkerWrapper(Worker::from(worker)));

    let test_command = serde_json::to_string(&WorkerCommand::StartDay(
        YearDay::new(2022, 1),
        Input::Default,
    ))
    .unwrap();

    JS_BRIDGE
        .lock()
        .unwrap()
        .borrow()
        .worker_wrapper
        .as_some()
        .post_message(&JsValue::from(test_command))
        .unwrap();
}

#[wasm_bindgen]
pub fn on_worker_message(msg: JsValue) {
    let progress: SolveProgress = serde_json::from_str(&msg.as_string().unwrap()).unwrap();
    executor::block_on(JS_BRIDGE.lock().unwrap().worker_tx.as_some().send(progress)).unwrap();
}

#[wasm_bindgen]
pub fn set_scale(scale: JsValue) {
    let scale = scale.as_f64().unwrap();
    JS_BRIDGE.lock().unwrap().scale = scale;
}

#[wasm_bindgen]
pub fn push_key_event(key: JsValue) {
    let key = key.as_string().unwrap();
    JS_BRIDGE.lock().unwrap().unhandled_keys.push(key);
}

#[wasm_bindgen]
pub fn main_wasm() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    main().map_err(|x| JsValue::from(format!("{:?}", x)))?;

    Ok(())
}
