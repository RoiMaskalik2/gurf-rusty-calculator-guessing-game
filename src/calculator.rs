//! calculator that can receive an operation and two i32 integers and perform the calculation according to the operation.
//! The valid operations of the calculator are: "+" - Addition, "-" - Subtraction, "*" - Multiplication, "/" - Division

use crate::{Error, Result};
use strum::{EnumIter, IntoEnumIterator};

/// This enum Represents all of the possible operations allowed in the calculator
#[derive(EnumIter)]
pub enum Operator {
    /// Addition math operation (+)
    Addition,

    /// Subtraction math operation (-)
    Subtraction,

    /// Multiplication math operation (*)
    Multiplication,

    /// Division math operation (/)
    Division,
}

impl Operator {
    // This function matches all of the Operator enum variants into a corresponding char that represents the operation
    fn symbol(&self) -> char {
        match self {
            Self::Addition => '+',
            Self::Subtraction => '-',
            Self::Multiplication => '*',
            Self::Division => '/',
        }
    }

    /// This function lists all of the symbols (characters) that are allowed as operations in the calculator
    pub fn list_symbols() -> String {
        let operations: Vec<String> = Self::iter()
            .map(|operator| operator.symbol().to_string())
            .collect();

        format!("({})", operations.join(", "))
    }
}

impl TryFrom<char> for Operator {
    type Error = Error;

    // This function converts a symbol into it's corresponding Operator variant
    fn try_from(value: char) -> Result<Self> {
        Self::iter()
            .find(|operator| operator.symbol() == value)
            .ok_or(Error::InvalidOperation)
    }
}

/// Performs a safe calculation according to the operation on the two parameters and returns an error in case of a failure
///
/// # Errors
///
/// This function will return an error if:
/// * A calculation results in an integer overflow
/// * A calculation results in a division by zero
pub fn perform_calculation(
    first_number: i32,
    second_number: i32,
    operator: Operator,
) -> Result<i32> {
    match operator {
        Operator::Addition => addition(first_number, second_number),
        Operator::Subtraction => subtraction(first_number, second_number),
        Operator::Multiplication => multiplication(first_number, second_number),
        Operator::Division => division(first_number, second_number),
    }
}

/// Performs a safe addition and returns an error in case of an integer overflow
fn addition(first_number: i32, second_number: i32) -> Result<i32> {
    first_number
        .checked_add(second_number)
        .ok_or(Error::AdditionOverflow)
}

/// Performs a safe subtraction and returns an error in case of an integer overflow
fn subtraction(first_number: i32, second_number: i32) -> Result<i32> {
    first_number
        .checked_sub(second_number)
        .ok_or(Error::SubtractionOverflow)
}

/// Performs a safe multiplication and returns an error in case of an integer overflow
fn multiplication(first_number: i32, second_number: i32) -> Result<i32> {
    first_number
        .checked_mul(second_number)
        .ok_or(Error::MultiplicationOverflow)
}

/// Performs a safe division and returns an error in case of an division by 0
fn division(first_number: i32, second_number: i32) -> Result<i32> {
    first_number
        .checked_div(second_number)
        .ok_or(Error::DivisionByZero)
}
