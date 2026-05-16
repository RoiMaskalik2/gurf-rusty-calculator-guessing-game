//! Errors that can occur while taking user input using the [crate::user_input] module.
use std::{char, fmt, io, num};

/// Represents an error that can occur while taking user input using the [crate::user_input] module.
#[derive(Debug, derive_more::From)]
#[allow(dead_code)]
pub enum Error {
    /// User provided an empty input.
    EmptyString,

    /// Error occurred during input reading.
    #[from]
    Io(io::Error),

    /// Error occurred during an input conversation to an integer.
    #[from]
    ParseInt(num::ParseIntError),
    /// Error occurred during an input conversation to a char.
    #[from]
    ParseChar(char::ParseCharError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
