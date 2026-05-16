//! friendly assistant for welcoming users.

use std::fmt;

/// Use this struct to print a nice welcome message
pub struct Rusty {
    /// The name of the user that will be greeted.
    member_name: String,
}

impl Rusty {
    /// Creates a new Rusty assistant instance.
    pub fn new(member_name: String) -> Self {
        Self { member_name }
    }
}

impl fmt::Display for Rusty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dear {}, Welcome To My First Rust Program!",
            self.member_name
        )
    }
}
