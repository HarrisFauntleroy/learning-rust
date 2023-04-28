use std::io;
use std::io::Write;

// Function to read and parse a f64 number from user input
fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush stdout to display the prompt before waiting for input
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

// Function to read an operator from user input
fn read_operator(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush stdout to display the prompt before waiting for input
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn error_message(error: &str) -> String {
    format!("Error: {}", error)
}

fn perform_operation(num1: f64, num2: f64, operator: &str) -> Result<f64, String> {
    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Ok(num1 / num2)
            } else {
                Err(error_message("Division by zero!"))
            }
        }
        _ => Err(error_message("Invalid operator!")),
    }
}

fn main() {
    let first_number = read_number("Enter the first number: ");
    let second_number = read_number("Enter the second number: ");
    let operator = read_operator("Enter an operator (+, -, *, /): ");

    match perform_operation(first_number, second_number, &operator) {
        Ok(result) => println!("{:.2} {} {:.2} = {:.2}", first_number, operator, second_number, result),
        Err(error) => println!("{}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(perform_operation(2.0, 3.0, "+").unwrap(), 5.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(perform_operation(5.0, 3.0, "-").unwrap(), 2.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(perform_operation(2.0, 3.0, "*").unwrap(), 6.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(perform_operation(6.0, 3.0, "/").unwrap(), 2.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(perform_operation(6.0, 0.0, "/").unwrap_err(), error_message("Division by zero!"));
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(perform_operation(6.0, 3.0, "x").unwrap_err(), error_message("Invalid operator!"));
    }
}
