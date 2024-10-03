use rustemo::Parser;
use std::io;
// Use the generated parser
use crate::arithmetic::ArithmeticParser;

// Include generated modules
#[rustfmt::skip]
mod arithmetic;
#[allow(unused)]
#[rustfmt::skip]
mod arithmetic_actions;

fn main() {
    let mut expression = String::new();

    // Read the line from the input
    println!("Expression:");
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    // Parse the line and get the result.
    let result = ArithmeticParser::new().parse(&expression).unwrap();

    println!("Result = {result}");
}
