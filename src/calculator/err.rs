//! Errors that can occur while using the [crate::calculator] module.
use std::fmt;

/// Represents an error that can occur while performing calculations using the [crate::calculator] module.
#[derive(Debug, derive_more::From)]
pub enum Error {
    /// Calculation was performed with an invalid operator.
    InvalidOperation,

    /// Overflow occured in Addition calculation.
    AdditionOverflow,

    /// Overflow occured in Subtraction calculation.
    SubtractionOverflow,

    /// Overflow occured in Multiplication calculation.
    MultiplicationOverflow,

    // Division By 0
    DivisionByZero,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
