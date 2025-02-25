use std::error::Error;
use std::fmt;
use std::io;

/// Used when Hyprland sends an error over a socket
#[derive(Debug, Clone)]
pub struct HyprlandError {
    message: String,
}

impl HyprlandError {
    pub fn new(message: String) -> Self {
        HyprlandError { message }
    }
}

impl fmt::Display for HyprlandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for HyprlandError {}

#[derive(Debug)]
pub enum CommandError {
    HyprlandError(HyprlandError),
    IOError(io::Error),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::HyprlandError(e) => Some(e),
            Self::IOError(e) => Some(e),
        }
    }
}
