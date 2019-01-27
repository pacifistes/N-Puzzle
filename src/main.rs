mod resolver;

use resolver::parser::parse;
use resolver::puzzle::*;
use resolver::resolver::*;
use resolver::generate::generate_random_puzzle;
use resolver::generate::generate_sorted_puzzle;
use std::env;

fn resolve(puzzle: Puzzle) {
    let size = puzzle.get_size();
    let goal: Puzzle = Puzzle::new(generate_sorted_puzzle(&size), size);
    let mut resolver: Resolver = Resolver::new(puzzle, goal);
    dbg!(&resolver);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle: Puzzle = match args.len() {
        2 => match parse(&args[1]) {
            Ok(puzzle) => puzzle,
            Err(err) => {
                println!("Error: {}", err);
                return;
            }
        },
        _ => generate_random_puzzle(),
    };
    match puzzle.is_solvable() {
        true => resolve(puzzle),
        false => println!("the puzzle is unsolvable")
    }
}
