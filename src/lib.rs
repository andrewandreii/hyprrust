//! A crate that provides a rust interface to communicate with the Hyprland sockets.
//!
//! There are two types of Hyprland sockets, one used for events and one used for commands or to
//! request data (ie. clients, decorations, etc.)
//!
//! The main struct of this crate is [`HyprlandConnection`]. It's used for connecting to both
//! sockets (except when you need sync events, in which case you should use [`DetachedEventConnection`]).
//!
//! [`DetachedEventConnection`]: crate::events::single_event_sync::DetachedEventConnection
//!
//! # Crate overview
//!
//! ### Get started
//!
//! By default, this crate enables both the `sync` and `async` features. It's preferred to disable
//! default features and include only the `sync` feature if you don't need `async`, this way
//! `tokio` is not included.
//!
//! To get started add this crate to your project:
//! ```sh
//! cargo add hyprrust
//! ```
//!
//! ### Module structure
//!
//! This crate conatins several modules:
//!  - `commands`: All implemented commands, but doesn't include arguments.
//!  - `commands::prelude`: All commands and arguments.
//!  - `data`: All commands used to request data from Hyprland.
//!  - `errors`: All errors returned by the API of this library.
//!  - `events`: Contains everything needed to receive sync and async events and filter them.
//!
//! ### Examples
//!
//! See the `examples` folder in the crates source.

mod connection;
mod ctl;
pub mod errors;
pub mod events;

pub use connection::HyprlandConnection;

pub mod commands {
    pub use crate::ctl::commands::*;

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
