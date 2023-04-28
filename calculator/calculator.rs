use std::io;
use std::io::Write;

fn get_number(prompt: &str) -> f64 {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn get_operator(prompt: &str) -> char {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().parse().expect("Invalid operator")
}

fn perform_operation(num1: f64, num2: f64, operator: char) -> Result<f64, String> {
    match operator {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => {
            if num2 != 0.0 {
                Ok(num1 / num2)
            } else {
                Err(String::from("Error: Division by zero!"))
            }
        }
        _ => Err(String::from("Error: Invalid operator!")),
    }
}

fn main() {
    let num1 = get_number("Enter the first number: ");
    let num2 = get_number("Enter the second number: ");
    let operator = get_operator("Enter an operator (+, -, *, /): ");

    match perform_operation(num1, num2, operator) {
        Ok(result) => println!("{:.2} {} {:.2} = {:.2}", num1, operator, num2, result),
        Err(error) => println!("{}", error),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn test_perform_operation_addition() {
    let result = perform_operation(5.0, 3.0, '+').unwrap();
    assert_eq!(result, 8.0);
}

#[test]
fn test_perform_operation_subtraction() {
    let result = perform_operation(5.0, 3.0, '-').unwrap();
    assert_eq!(result, 2.0);
}

#[test]
fn test_perform_operation_multiplication() {
    let result = perform_operation(5.0, 3.0, '*').unwrap();
    assert_eq!(result, 15.0);
}

#[test]
fn test_perform_operation_division() {
    let result = perform_operation(6.0, 3.0, '/').unwrap();
    assert_eq!(result, 2.0);
}

#[test]
#[should_panic(expected = "Error: Division by zero!")]
fn test_perform_operation_division_by_zero() {
    perform_operation(5.0, 0.0, '/').unwrap();
}

#[test]
#[should_panic(expected = "Error: Invalid operator!")]
fn test_perform_operation_invalid_operator() {
    perform_operation(5.0, 3.0, '%').unwrap();
}
}
