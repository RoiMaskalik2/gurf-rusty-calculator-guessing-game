//! User input handling module.
//!
//! includes functions for  user input from the command line.

mod err;

use std::io;

pub use err::Error;
pub type Result<T> = core::result::Result<T, Error>;

/// Reads a string from the standard input, trims it, and checks that the input is not empty.
pub fn input_string(user_input: &mut String, display_message: &str) -> Result<()> {
    println!("{}", display_message);
    io::stdin().read_line(user_input)?;
    user_input.truncate(user_input.trim_end().len());

    (!user_input.is_empty())
        .then_some(())
        .ok_or(Error::EmptyString)?;

    Ok(())
}
