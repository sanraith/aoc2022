use futures::{stream, Future, Stream};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
    thread,
};

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

pub struct StreamState {
    pub items: Vec<ProgressEnum>,
    pub new_items_event: FutureEvent,
    new_items_mutex: Arc<Mutex<bool>>,
}
impl StreamState {
    pub fn new() -> StreamState {
        let mutex = Arc::new(Mutex::new(false));
        StreamState {
            items: Default::default(),
            new_items_mutex: Arc::clone(&mutex),
            new_items_event: FutureEvent::new(Arc::clone(&mutex)),
        }
    }

    pub fn reset_new_items_event(&mut self) {
        self.new_items_event = FutureEvent::new(Arc::clone(&self.new_items_mutex));
    }
}

#[derive(Clone)]
pub struct FutureEvent {
    pub ready_mutex: Arc<Mutex<bool>>,
}
impl Future for FutureEvent {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match *self.ready_mutex.lock().unwrap() {
            true => Poll::Ready(()),
            false => Poll::Pending,
        }
    }
}
impl FutureEvent {
    pub fn new(ready_mutex: Arc<Mutex<bool>>) -> FutureEvent {
        FutureEvent { ready_mutex }
    }

    pub fn fire(&mut self) {
        *self.ready_mutex.lock().unwrap() = true;
    }
}

pub trait SolutionRunner {
    fn solve() -> Box<dyn Stream<Item = ProgressEnum>>;
}
pub struct ThreadSolutionRunner {}
impl SolutionRunner for ThreadSolutionRunner {
    fn solve() -> Box<dyn Stream<Item = ProgressEnum>> {
        let state = Arc::new(Mutex::new(StreamState::new()));

        let thread_state = Arc::clone(&state);
        thread::spawn(move || {
            let mut state = thread_state.lock().unwrap();
            state.items.push(ProgressEnum::Progress(0.0));
            state.items.push(ProgressEnum::Progress(1.0));
            state.items.push(ProgressEnum::Done);
            state.new_items_event.fire();
        });

        let stream = stream::unfold(state, |state| async move {
            // Wait until items are available
            {
                let mut state = state.lock().unwrap();
                if state.items.len() == 0 {
                    state.new_items_event.clone().await;
                    state.reset_new_items_event();
                }
            }

            // Consume until done
            let item = state.lock().unwrap().items.pop();
            match item {
                Some(item) => match item {
                    ProgressEnum::Progress(_) => Some((item, state)),
                    ProgressEnum::Done => None,
                },
                None => panic!("Should not continue if there are no items to consume!"),
            }
        });

        Box::new(stream)
    }
}
