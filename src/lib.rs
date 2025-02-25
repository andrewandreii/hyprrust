mod all_events;
mod connection;
mod ctl;
pub mod errors;
pub mod events;

pub use connection::HyprlandConnection;

pub mod commands {
    pub use crate::ctl::command::*;

    pub mod prelude {
        pub use super::*;
        pub use crate::arguments::*;
    }
}

pub mod data {
    pub use crate::ctl::data::*;
}

pub mod arguments {
    pub use crate::ctl::arguments::*;
}
