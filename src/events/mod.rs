mod all_events;
mod common;

#[cfg(feature = "async")]
mod events_conn;

pub use all_events::{HyprlandEvent, HyprlandEventType};
pub use common::EventFilter;

#[cfg(feature = "async")]
pub mod single_event;

#[cfg(feature = "sync")]
pub mod single_event_sync;
