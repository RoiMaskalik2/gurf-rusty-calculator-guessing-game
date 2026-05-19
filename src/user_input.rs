//! User input handling module.
//!
//! includes functions for  user input from the command line.

use crate::{Error, Result};
use std::io;

/// Reads a string from the standard input, trims it, and checks that the input is not empty.
pub fn input_string(display_message: &str) -> Result<String> {
    println!("{}", display_message);

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;
    user_input.truncate(user_input.trim_end().len());

    (!user_input.is_empty())
        .then_some(())
        .ok_or(Error::EmptyString)?;

    Ok(user_input)
}

/// Reads an input from the standard input, trims it
/// validates that the input is an i32 type and returns the input as an integer
pub fn input_integer(display_message: &str) -> Result<i32> {
    let user_input = input_string(display_message)?;

    let integer_input = user_input.parse()?;
    Ok(integer_input)
}

/// Reads an input from the standard input, trims it
/// validates that the input is an char type and returns the input as a char
pub fn input_char(display_message: &str) -> Result<char> {
    let user_input = input_string(display_message)?;

    let char_input = user_input.parse()?;
    Ok(char_input)
}
