mod resolver;
mod tests;

use resolver::generate::generate_random_puzzle;
use resolver::generate::generate_sorted_puzzle;
use resolver::heuristic::*;
use resolver::parser::parse;
use resolver::puzzle::*;
use resolver::resolver::*;
use std::env;
use std::time::Instant;


fn run(puzzle: Puzzle) {
    let size = puzzle.get_size();
    let goal: Puzzle = Puzzle::new(generate_sorted_puzzle(size), size, 0);

    match puzzle.is_solvable(&goal) {
        true => {
            println!("the puzzle is solvable");
            let mut resolver: Resolver = Resolver::new(puzzle, goal);
            // dbg!(resolver.resolve());
			let start = Instant::now();
			resolver.resolve();
			let elapsed = start.elapsed();
			// loop {}
			resolver.print();
        	println!("time for resolve {:?}", elapsed);
        }
        false => println!("the puzzle is unsolvable"),
    }
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
    run(puzzle);
}