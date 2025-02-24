mod connection;
pub mod ctl;
pub mod events;

pub use connection::{HyprlandConnection, HyprlandError};

use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct LibraryError {
    message: String,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LibraryError {}
