//! Errors that can occur while taking user input using the [`user_input`] module.
use std::io;

//! Errors that can occur while taking user input using the [crate::user_input] module.
use std::{char, fmt, io, num};

/// Represents an error that can occur while taking user input using the [crate::user_input] module.
#[derive(Debug, derive_more::From)]
#[allow(dead_code)]
pub enum Error {
    /// User provided an empty input.
    #[error("{self:?}")]
    EmptyString,

    /// Error occurred during input reading.
    #[error("{self:?}")]
    Io(#[from] io::Error),

    /// Error occurred during an input conversation to an integer.
    #[error("{self:?}")]
    ParseInt(#[from] num::ParseIntError),
    /// Error occurred during an input conversation to a char.
    #[error("{self:?}")]
    ParseChar(#[from] char::ParseCharError),
}
