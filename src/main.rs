#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Rusty, Calculator and Guesser application.
//!
//! This application includes 3 exercises (Didnt have power to create different repo for each dir Gurf please don't kill me):
//! 1. Rusty: take user name by input and print a nice message
//! 2. Calculator: take two numbers and a valid operator by input and perform a calculation
//! 3. Guesser: TBD
use rusty_calculator_guesser::{Rusty, calculator, user_input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Rusty
    run_rusty()?;

    // Calculator
    run_calculator()?;

    // Guesser - TBD

    Ok(())
}

fn run_rusty() -> Result<(), Box<dyn std::error::Error>> {
    let user_name = user_input::input_string("Please Insert Your Name")?;

    let rusty_assistant = Rusty::new(user_name);
    println!("{rusty_assistant}");

    Ok(())
}

fn run_calculator() -> Result<(), Box<dyn std::error::Error>> {
    let first_number = user_input::input_integer("Enter First Number:")?;
    let second_number = user_input::input_integer("Enter Second Number:")?;

    let operation_display_message =
        format!("Choose Operation {}:", calculator::Operator::list_symbols());
    let operator = user_input::input_char(&operation_display_message)?;
    let operator = calculator::Operator::try_from(operator)?;

    let calculation = calculator::perform_calculation(first_number, second_number, operator)?;

    print!("Result: {}", calculation);

    Ok(())
}
