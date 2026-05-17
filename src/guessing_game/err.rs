//! Errors that can occur while taking user input using the [crate::guessing_game] module.
use crate::user_input;
use std::fmt;

/// Represents an error that can occur while using the [crate::guessing_game] module.
#[derive(Debug, derive_more::From)]
#[allow(dead_code)]
pub enum Error {
    /// User provided a number not in range of the game.
    InvalidGuessRange,

    /// The implementor using the [crate::guessing_game::GuessingGameEngine]
    /// did not initialize the engine before attempting to take an input from the user
    UninitializedGame,

    /// Overflow occured from taking too many guesses without restarting the guessing game
    TotalTurnsOverflow,

    /// Error occurred during input reading.
    #[from]
    Io(user_input::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
