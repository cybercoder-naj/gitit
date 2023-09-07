pub mod cursor;
pub mod state;
pub mod models;


use std::sync::{Arc, Mutex, MutexGuard};
use event_emitter_rs::EventEmitter;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EVENT_EMITTER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}

pub fn event_emitter() -> MutexGuard<'static, EventEmitter> {
    EVENT_EMITTER.lock().unwrap()
}

pub trait KeypressListener {
    fn on_keypress(this: Arc<Mutex<Self>>);
}