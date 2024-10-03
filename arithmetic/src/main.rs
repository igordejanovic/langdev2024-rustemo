use rustemo::Parser;
use std::io;
// Use the generated parser and builder
use crate::arithmetic::{ArithmeticParser, DefaultBuilder};

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

    // Evaluate each tree from the forest
    let results = forest
        .into_iter()
        .map(|tree| {
            let mut builder = DefaultBuilder::new();
            tree.build(&mut builder)
        })
        .collect::<Vec<_>>();

    println!("{results:?}");
}
