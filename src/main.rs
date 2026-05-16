#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Rusty, Calculator and Guesser application.
//!
//! This application includes 3 exercises (Didnt have power to create different repo for each dir Gurf please don't kill me):
//! 1. Rusty: take user name by input and print a nice message
//! 2. Calculator: TBD
//! 3. Guesser: TBD

mod rusty;
mod user_input;

use rusty::Rusty;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rusty
    let mut user_name = String::new();
    user_input::input_string(&mut user_name, "Please Insert Your Name")?;

    let rusty_assistant = Rusty::new(user_name);
    println!("{rusty_assistant}");

    // Calculator - TBD

    // Guesser - TBD
    Ok(())
}
