use std::io;

fn read_number(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn read_operator(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
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
                Err("Error: Division by zero!".to_string())
            }
        }
        _ => Err("Error: Invalid operator!".to_string()),
    }
}

fn main() {
    let num1 = read_number("Enter the first number: ");
    let num2 = read_number("Enter the second number: ");
    let operator = read_operator("Enter an operator (+, -, *, /): ");

    match perform_operation(num1, num2, &operator) {
        Ok(result) => println!("{:.2} {} {:.2} = {:.2}", num1, operator, num2, result),
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
        assert_eq!(perform_operation(6.0, 0.0, "/").unwrap_err(), "Error: Division by zero!");
    }

    #[test]
    fn test_invalid_operator() {
        assert_eq!(perform_operation(6.0, 3.0, "x").unwrap_err(), "Error: Invalid operator!");
    }
}
