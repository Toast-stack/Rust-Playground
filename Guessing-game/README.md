# Rust Guessing Game
## Overview
The Rust Guessing Game is a simple command-line application where the program generates a random number, and the user attempts to guess it. The program provides feedback on whether the guess is too high, too low, or correct. This project is based on the [Rust Book's Guessing Game tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) and demonstrates key Rust concepts like user input, random number generation, and control flow.

## Features
1. Generates a random number between 1 and 100.
2. Accepts user input for guesses.
3. Provides feedback whether the guess is too high, too low, or correct.
4. Handles invalid input gracefully.

## Prerequisites
### Software Requirements
  - **Rust**: Installed via [rustup](https://www.rust-lang.org/tools/install).
  - A terminal or command-line interface to run the program.

## Setup Instructions
1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd Guessing-game
   ```
2. Build the project:
   ```Bash
   cargo build
   ```
3. Run the application:
   ```Bash
   cargo run
   ```
## How to play
1. Run the program using `cargo run`.
2. The program will prompt you to guess a number between 1 and 100.
3. Enter your guess and press Enter.
4. The program will provide feedback:
   - If your guess is too high, it will say "Too big!"
   - If your guess is too low, it will say "Too small!"
   - If your guess is correct, it will congratulate you and exit.
5. If you enter invalid input (e.g., a non-numeric value), the program will prompt you to try again.

## Code Highlights
### Key Concepts Demonstrated
1. Random Number Generation:
   - Uses the `rand` crate to generate a random number.
2. User Input:
   - Reads input from the user using `std::io`.
3. Error Handling:
   - Handles invalid input gracefully using `Result` and `expect`.
4. Control Flow:
   - Uses `loop`, `match`, and conditional statements for game logic.
  
## Example Code
Here's a snippet of the main game loop:
```Rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Customization Ideas
1. Change the Range:
   - Modify the range of the random number (e.g., 1 to 1000).
2. Add a Guess Counter:
   - Track the number of guesses and display it when the user wins.
3. Difficulty Levels:
   - Add easy, medium, and hard modes with different ranges or limited attempts.
  
## References
1. [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html): Official guide for learning Rust, including the Guessing Game tutorial.
2. [rand Crate Documentation](https://docs.rs/rand/latest/rand/): Documentation for the `rand` crate used for random number generation.
3. [Rust Standard Library](https://doc.rust-lang.org/std/): Comprehensive reference for Rust's standard library.
