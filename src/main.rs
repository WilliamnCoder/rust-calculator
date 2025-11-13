use std::{io, result};
fn main() {
    println!("Welcome to the Calculator App written in Rust!");
    
    // declare mutable string input
    let mut number1 = String::new();
    let mut number2 = String::new();
    let mut op =String::new();


    println!("Enter a number: ");
    io::stdin().read_line(&mut number1).unwrap(); // reading lines from io:stdin

    println!("Enter a second number: ");
    io::stdin().read_line(&mut number2).unwrap();
    
    println!("Choose an operator from the list (type in the operator): ");
    println!("+ Addition");
    println!("- Subtraction");
    println!("* Multiplication");
    println!("/ Division");
    io::stdin().read_line(&mut op).expect("Input is invalid");


    //convert strings to f64 and char
    let number1: f64 = number1.trim().parse().unwrap(); // parse variables into f64
    let number2: f64 = number2.trim().parse().unwrap(); 
    let op: char = op.trim().chars().next().unwrap();

    let result = calculate(number1, number2, op);
    println!("Result: {}", result);

    }

    fn calculate(number1: f64, number2: f64, op: char) -> f64 {
    match op {
        '+' => number1 + number2,
        '-' => number1 - number2,
        '*' => number1 * number2,
        '/' => number1 / number2,
        _ => {
            println!("Error: unknown operator");
            0.0
            }
        }   
    }