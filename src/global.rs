pub mod cursor;
pub mod state;
pub mod models;


use std::sync::{Mutex, MutexGuard};
use event_emitter_rs::EventEmitter;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EVENT_EMITTER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}

pub fn event_emitter() -> MutexGuard<'static, EventEmitter> {
    EVENT_EMITTER.lock().unwrap()
}