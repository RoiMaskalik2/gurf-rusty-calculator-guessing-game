//! Errors that can occur while taking user input using the [`user_input`] module.
use std::io;

/// Represents an error that can occur while taking user input using the [`user_input`] module.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// User provided an empty input.
    #[error("{self:?}")]
    EmptyString,

    /// Error occurred during input reading.
    #[error("{self:?}")]
    Io(#[from] io::Error),
}
