use std::io;

// Reads an integer from standard input after printing the given prompt.
// It loops until the user enters a valid integer.
fn read_int(prompt: &str) -> i32 {
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

// Reads a string (e.g., operation choice) from standard input after printing the given prompt.
fn read_str(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    // Convert the input to lowercase so it's easier to match.
    input.trim().to_lowercase()
}

// Prints an initial message to the user.
fn print_greeting() {
    println!("Top of the morning to ya! Let's do some math!");
}

// Returns the sum of x and y.
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Returns the difference of x minus y.
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

// Returns the product of x and y.
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Returns the quotient of x divided by y (integer division).
fn divide(x: i32, y: i32) -> i32 {
    x / y
}

fn main() {
    // Greet the user
    print_greeting();

    loop {
        // 1) Get two integers from the user (now robust against invalid input).
        let x = read_int("Please enter a value for x: ");
        let y = read_int("Please enter a value for y: ");

        // 2) Ask which operation they want to perform. If it's invalid, we loop again.
        let operation = read_str("Which operation? (a = Add, s = Subtract, m = Multiply, d = Divide)");

        let result = match operation.as_str() {
            "a" => add(x, y),
            "s" => subtract(x, y),
            "m" => multiply(x, y),
            "d" => {
                // Check for division by zero
                if y == 0 {
                    println!("Cannot divide by zero. Please try again.");
                    continue; // Go back to the top of the loop
                }
                divide(x, y)
            }
            _ => {
                println!("Unrecognized operation. Please choose 'a', 's', 'm', or 'd'.");
                continue; // Skip the rest of this loop iteration and prompt again
            }
        };

        println!("The result is: {}", result);

        // 3) Ask if the user wants to continue or exit.
        let choice = read_str("Would you like to do another operation? (y/n)");
        if choice == "n" {
            println!("Thanks for using the calculator. Goodbye!");
            break;
        }
    }
}
