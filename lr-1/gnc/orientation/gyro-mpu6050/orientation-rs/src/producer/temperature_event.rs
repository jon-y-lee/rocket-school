use serde::{Deserialize, Serialize};
use crate::producer::event_type::EventType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event<T> {
    pub eventType: T,
    pub value: f32
}