mod all_events;
mod common;
mod events_conn;

pub use all_events::HyprlandEvent;
pub use common::{event_name, EventFilter};

pub mod single_event;
pub mod single_event_sync;
