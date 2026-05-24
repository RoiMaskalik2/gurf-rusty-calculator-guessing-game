//! Guessing Game engine that exports an api for a user to build a guessing game
//! It allows the implementor to do the following things:
//! * start a game where a random number is generated
//! * take input from the user and return an indication of the user's number guess
//! * reset the game engine in order to start a new game

use crate::{Error, Result};
use const_format::formatcp;
use rand::RngExt;
use std::{cmp::Ordering, ops::RangeInclusive};

/// The guessing game engine generates a random number between this range.
///  this is the range of numbers that the user is allowd to guess between
const GUESS_RANGE: RangeInclusive<i32> = 1..=100;

/// A message that will be diaplyed to the user when expecting to take an stdinput from the user.
pub const DISPLAY_MESSAGE: &str = formatcp!(
    "Insert a number between {} - {}",
    *GUESS_RANGE.start(),
    *GUESS_RANGE.end()
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
    /// A random generated number that the user is supposed to guess
    /// Becomes None After the user has managed to guess the number
    generated_number: Option<i32>,

    /// The amount of turns it took the user to guess the generated number
    total_turns: u32,
}

impl GuessingGameEngine {
    /// Constructor for the game engine.
    /// initializes a new guessing game with a random generated number
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            generated_number: Some(rng.random_range(GUESS_RANGE)),
            total_turns: 0,
        }
    }

    /// Getter for the total rounds of the guessing game
    pub fn get_total_turns(&self) -> u32 {
        self.total_turns
    }

    /// Run one iteration of the guessing game - take a numeric input from stdin.
    /// Return to the user if the number was higher/lower than the generated number
    pub fn take_guess(&mut self, user_guess: i32) -> Result<GuessOutcome> {
        GUESS_RANGE
            .contains(&user_guess)
            .then_some(())
            .ok_or(Error::InvalidGuessRange)?;

        let generated = self.generated_number.ok_or(Error::TakeGuessAfterGameEnd)?;

        self.increment_total_turns()?;

        let guess_outcome = GuessOutcome::from(user_guess.cmp(&generated));

        if let GuessOutcome::Correct = guess_outcome {
            self.generated_number = None;
        }

        Ok(guess_outcome)
    }

    // This helper method is being called to represent that a guess was taken, meaning it increases the amount of total guesses.
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
