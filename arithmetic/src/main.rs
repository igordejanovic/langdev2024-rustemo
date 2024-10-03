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

    // Parse the line and get all results.
    let forest = ArithmeticParser::new().parse(&expression).unwrap();

    println!("Number of interpretations = {}", forest.solutions());

    for (tree_idx, tree) in forest.iter().enumerate() {
        println!("**** TREE {tree_idx}");
        println!("{tree:#?}");
    }
}
