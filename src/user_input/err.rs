//! Errors that can occur while taking user input using the [`user_input`] module.

use std::fmt;
use std::io;

/// Represents an error that can occur while taking user input using the [`user_input`] module.
#[derive(Debug, derive_more::From)]
pub enum Error {
    /// User provided an empty input.
    EmptyString,

    /// Error occurred during input reading.
    #[from]
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyString => write!(f, "input cannot be empty"),
            Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for Error {}
