//! Guessing Game engine that exports an api for a user to build a guessing game
//! It allows the implementor to do the following things:
//! * start a game where a random number is generated
//! * take input from the user and return an indication of the user's number guess
//! * reset the game engine in order to start a new game

use crate::{Error, Result};
use const_format::formatcp;
use rand::RngExt;
use std::{cmp::Ordering, ops::Range};

/// The guessing game engine generates a random number between this range.
///  this is the range of numbers that the user is allowd to guess between
const GUESS_RANGE: Range<i32> = 1..101;

/// A message that will be diaplyed to the user when expecting to take an stdinput from the user.
pub const DISPLAY_MESSAGE: &str = formatcp!(
    "Insert a number between {} - {}",
    GUESS_RANGE.start,
    (GUESS_RANGE.end - 1)
);

/// Enum representing the outcome of a guess
pub enum GuessOutcome {
    /// The guess was correct - NOTE: The game will end in that case
    Correct,

    /// The guess was too high - The user should guess a lower number
    TooHigh,

    /// The guess was too low - The user should guess a higher number
    TooLow,
}

impl From<Ordering> for GuessOutcome {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Equal => Self::Correct,
            Ordering::Less => Self::TooLow,
            Ordering::Greater => Self::TooHigh,
        }
    }
}

/// The Guessing Game engine, Createing a Guessing Game engine will allow the implementor to build a guessing game
pub struct GuessingGameEngine {
    /// Determines whether a game is currently running (true) or ended (false)
    is_game_active: bool,

    /// A random generated number that the user is supposed to guess
    generated_number: i32,

    /// The amount of turns it took the user to guess the generated number
    total_turns: u32,
}

impl GuessingGameEngine {
    /// Constructor for the game engine.
    /// initializes a new guessing game with a random generated number
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            is_game_active: true,
            generated_number: rng.random_range(GUESS_RANGE),
            total_turns: 0,
        }
    }

    /// This function allows the game engine to start a new guessing game
    /// It generates a new number and resets the state of the game
    #[allow(dead_code)]
    pub fn start_new_game(&mut self) {
        let mut rng = rand::rng();

        self.generated_number = rng.random_range(GUESS_RANGE);
        self.is_game_active = true;
        self.total_turns = 0;
    }

    /// Getter for the total rounds of the guessing game
    pub fn get_total_turns(&self) -> u32 {
        self.total_turns
    }

    /// Run one iteration of the guessing game - take a numeric input from stdin.
    /// Return to the user if the number was higher/lower than the generated number
    pub fn take_guess(&mut self, user_guess: i32) -> Result<GuessOutcome> {
        if !self.is_game_active {
            return Err(Error::UninitializedGame);
        }

        GUESS_RANGE
            .contains(&user_guess)
            .then_some(())
            .ok_or(Error::InvalidGuessRange)?;

        self.increment_total_turns()?;

        let guess_outcome = GuessOutcome::from(user_guess.cmp(&self.generated_number));

        // End the game if the guess is correct
        if let GuessOutcome::Correct = guess_outcome {
            self.is_game_active = false;
        }

        Ok(guess_outcome)
    }

    fn increment_total_turns(&mut self) -> Result<()> {
        self.total_turns = self
            .total_turns
            .checked_add(1)
            .ok_or(Error::TotalTurnsOverflow)?;
        Ok(())
    }
}

impl Default for GuessingGameEngine {
    fn default() -> Self {
        Self::new()
    }
}
