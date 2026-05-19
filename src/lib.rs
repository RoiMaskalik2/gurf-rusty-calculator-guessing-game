#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Rusty, Calculator and Guesser application.
//!
//! Exports the building blocks for 3 exercises: (Didnt have power to create different repo for each dir Gurf please don't kill me):
//! 1. [`rusty`]: take user name by input and print a nice message.
//! 2. [`calculator`]: take two numbers and a valid operator by input and perform a calculation.
//! 3. guessing_game - TBD

pub mod calculator;
mod err;
pub mod rusty;
pub mod user_input;

/// Rusty exercise module re-exports
pub use rusty::Rusty;

/// Calculator exercise module re-exports
pub use calculator::Operator;

pub use err::Error;
/// Type alias for the Result enum so that callers will not need to include the error enum in it.
pub type Result<T> = core::result::Result<T, Error>;
