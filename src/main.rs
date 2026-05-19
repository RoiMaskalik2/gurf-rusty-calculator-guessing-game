use rusty_calculator_guesser::{
    GuessOutcome, GuessingGameEngine, Operator, Result, Rusty, calculator, guessing_game,
    user_input,
};

fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    // Rusty
    run_rusty()?;

    // Calculator
    run_calculator()?;

    // Guesser - TBD
    run_guessing_game()?;
    Ok(())
}

fn run_rusty() -> Result<()> {
    let user_name = user_input::input_string("Please Insert Your Name")?;

    let rusty_assistant = Rusty::new(user_name);
    println!("{rusty_assistant}");

    Ok(())
}

fn run_calculator() -> Result<()> {
    let first_number = user_input::input_integer("Enter First Number:")?;
    let second_number = user_input::input_integer("Enter Second Number:")?;

    let operation_display_message =
        format!("Choose Operation {}:", calculator::Operator::list_symbols());
    let operator = user_input::input_char(&operation_display_message)?;
    let operator = Operator::try_from(operator)?;

    let calculation = calculator::perform_calculation(first_number, second_number, operator)?;

    println!("Result: {}", calculation);

    Ok(())
}

/// Run the guessing game exercise - generate a random number and let the user guess (with input from stdin) the number until a match.
pub fn run_guessing_game() -> Result<()> {
    let mut guessing_game_engine = GuessingGameEngine::new();

    loop {
        let user_guess = user_input::input_integer(guessing_game::DISPLAY_MESSAGE)?;

        match guessing_game_engine.take_guess(user_guess)? {
            GuessOutcome::TooHigh => println!("Your Guess Is Higher"),
            GuessOutcome::TooLow => println!("Your Guess Is Lower"),
            GuessOutcome::Correct => {
                println!(
                    "Game Won! It took you {} turns",
                    guessing_game_engine.get_total_turns()
                );

                // The game has ended, therefore we can stop the loop
                break;
            }
        }
    }

    Ok(())
}
