mod resolver;
mod tests;

use resolver::generate::generate_random_puzzle;
use resolver::generate::generate_sorted_puzzle;
use resolver::heuristic::*;
use resolver::parser::parse;
use resolver::puzzle::*;
use resolver::resolver::*;
use std::env;

fn run(puzzle: Puzzle) {
    let size = puzzle.get_size();
    let goal: Puzzle = Puzzle::new(generate_sorted_puzzle(size), size, 0);

    match puzzle.is_solvable(&goal) {
        true => {
            println!("the puzzle is solvable");
            let mut resolver: Resolver =
                Resolver::new(puzzle, goal);
            dbg!(resolver.resolve());
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
    // criterion_group!(benches, criterion_benchmark);
    // criterion_main!(benches);
}

// use criterion::Criterion;

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(20)));
// }
