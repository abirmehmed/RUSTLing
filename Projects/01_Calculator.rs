use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    loop {
        println!("\nPlease select an operation:");
        println!("1. Addition (+)");
        println!("2. Subtraction (-)");
        println!("3. Multiplication (*)");
        println!("4. Division (/)");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 5.");
                continue;
            }
        };

        if choice == 5 {
            println!("Exiting the calculator. Goodbye!");
            break;
        }

        println!("Enter the first number:");
        let num1 = get_number();

        println!("Enter the second number:");
        let num2 = get_number();

        match choice {
            1 => println!("Result: {} + {} = {}", num1, num2, num1 + num2),
            2 => println!("Result: {} - {} = {}", num1, num2, num1 - num2),
            3 => println!("Result: {} * {} = {}", num1, num2, num1 * num2),
            4 => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                } else {
                    println!("Result: {} / {} = {}", num1, num2, num1 / num2);
                }
            }
            _ => println!("Invalid choice. Please select a valid operation."),
        }
    }
}

fn get_number() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
