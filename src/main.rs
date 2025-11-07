use std::io;
fn main() {
    println!("Welcome to the Calculator App written in Rust!");
    println!("Enter expression (two numbers, one per line): ");
    // declare mutable string input
    let mut number1 = String::new();
    let mut number2 = String::new();

    // reading lines from io:stdin
    io::stdin().read_line(&mut number1).unwrap();
    io::stdin().read_line(&mut number2).unwrap();

    //parse variables into f64
    let number1: f64 = number1.trim().parse().unwrap();
    let number2: f64 = number2.trim().parse().unwrap();

    //print variables
    println!("{}", number1);
    println!("{}", number2); 
}
