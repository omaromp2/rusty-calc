// src/main.rs

mod input;       // Tells Rust to look in src/input.rs
mod calc;  // Tells Rust to look in src/calculator.rs

use input::{print_greeting, read_int, read_str};
use calc::Calc;

fn main() {
    print_greeting();

    loop {
        let x = read_int("Please enter a value for x: ");
        let y = read_int("Please enter a value for y: ");

        // Create a calculator "instance" with x and y
        let calc = Calc::new(x, y);

        // Ask user for operation
        let operation = read_str(
            "Which operation? (a = Add, s = Subtract, m = Multiply, d = Divide)",
        );

        // Perform the chosen operation
        match operation.as_str() {
            "a" => {
                let result = calc.add();
                println!("Result of {} + {} = {}", calc.x, calc.y, result);
            }
            "s" => {
                let result = calc.subtract();
                println!("Result of {} - {} = {}", calc.x, calc.y, result);
            }
            "m" => {
                let result = calc.multiply();
                println!("Result of {} * {} = {}", calc.x, calc.y, result);
            }
            "d" => {
                // Check for divide-by-zero
                match calc.divide() {
                    Some(result) => println!("Result of {} / {} = {}", calc.x, calc.y, result),
                    None => println!("Cannot divide by zero! Try again."),
                }
            }
            _ => {
                println!("Unrecognized operation. Please choose 'a', 's', 'm', or 'd'.");
                // Go back to the top of the loop without printing a result
                continue;
            }
        }

        // Ask if the user wants to continue
        let choice = read_str("Would you like to do another operation? (y/n)");
        if choice == "n" {
            println!("Thanks for using the calculator. Goodbye!");
            break;
        }
    }
}
