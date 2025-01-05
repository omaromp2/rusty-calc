// src/input.rs
use std::io;

/// Prints a greeting message.
pub fn print_greeting() {
    println!("Top of the morning to ya! Let's do some math!");
}

/// Reads an integer from standard input after printing the given prompt.
/// Loops until a valid integer is provided.
pub fn read_int(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid integer.");
            }
        }
    }
}

/// Reads a string (e.g., operation choice) from standard input after printing the given prompt.
/// Loops until a non-empty string is provided (you can tweak this logic to suit your needs).
pub fn read_str(prompt: &str) -> String {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let trimmed = input.trim();
        if !trimmed.is_empty() {
            return trimmed.to_lowercase();
        } else {
            println!("Please enter a valid string.");
        }
    }
}

