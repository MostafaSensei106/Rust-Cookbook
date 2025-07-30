//! A simple number guessing game.
//!
//! # Overview
//!
//! This is a basic terminal-based guessing game implemented in Rust.
//! The game generates a secret number between 1 and 100,
//! then continuously prompts the user to guess until they find the correct number.
//!
//! Colored output is used to enhance the user experience,
//! and invalid input is gracefully handled without crashing.
//!
//! # Features
//!
//! - Random number generation using the `rand` crate
//! - User input with validation
//! - Feedback on whether the guess is too high or too low
//! - Colored terminal output via the `colored` crate
//!
//! # Example
//!
//! ```bash
//! $ cargo run
//! Guess the number!
//! Please input your guess:
//! 50
//! Too small!
//! ...
//! You win! Good job!
//! ```

use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// The entry point of the guessing game.
///
/// This function:
/// - Greets the user
/// - Generates a random number between 1 and 100
/// - Loops until the user guesses the correct number
/// - Provides feedback based on the user's guess
///
/// # Panics
///
/// Panics if there is an error reading from stdin, although this is unlikely in typical use.
///
/// # Example
///
/// ```
/// // Run the program and follow the prompts
/// ```
fn main() {
    println!("{}", "Guess the number!".yellow());

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::rng().random_range(1..=100);

    // Uncomment the line below for debugging:
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("{}", "Please input your guess:".yellow());

        // Create a mutable String to store user input
        let mut guess = String::new();

        // Read the user's guess from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Attempt to parse the input into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input and restart loop
        };

        println!("You guessed: {}", guess);

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{} {}", "You win!".green(), "Good job!".green());
                break;
            }
        }
    }
}
