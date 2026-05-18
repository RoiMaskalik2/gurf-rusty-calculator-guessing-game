//! Errors that can occur while taking user input using the [crate::user_input] module.
use std::{char, io, num};

/// Represents an error that can occur while taking user input using the [crate::user_input] module.
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum Error {
    // ---- user_input ------------------------------------------------
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

    // ---- calculator ------------------------------------------------
    /// Calculation was performed with an invalid operator.
    #[error("{self:?}")]
    InvalidOperation,

    /// Overflow occured in Addition calculation.
    #[error("{self:?}")]
    AdditionOverflow,

    /// Overflow occured in Subtraction calculation.
    #[error("{self:?}")]
    SubtractionOverflow,

    /// Overflow occured in Multiplication calculation.
    #[error("{self:?}")]
    MultiplicationOverflow,

    /// Division By 0
    #[error("{self:?}")]
    DivisionByZero,

    // ---- calculator ------------------------------------------------
    /// Calculation was performed with an invalid operator.
    #[error("{self:?}")]
    InvalidOperation,

    /// Overflow occured in Addition calculation.
    #[error("{self:?}")]
    AdditionOverflow,

    /// Overflow occured in Subtraction calculation.
    #[error("{self:?}")]
    SubtractionOverflow,

    /// Overflow occured in Multiplication calculation.
    #[error("{self:?}")]
    MultiplicationOverflow,

    /// Division By 0
    #[error("{self:?}")]
    DivisionByZero,

    // ---- guessing_game ---------------------------------------------
    /// User provided a number not in range of the game.
    #[error("{self:?}")]
    InvalidGuessRange,

    /// The implementor using the module did not initialize the engine
    /// before attempting to take an input from the user
    #[error("{self:?}")]
    UninitializedGame,

    /// Overflow occured from taking too many guesses without restarting the guessing game
    #[error("{self:?}")]
    TotalTurnsOverflow,
}

impl std::error::Error for Error {}
