mod class;
mod heuristic;
mod parser;
mod resolver;

use class::{
    coordonnee::*,
    puzzle::{Puzzle, Square},
};
use heuristic::*;
use parser::puzzle::*;
use resolver::puzzle::Resolver;
use std::env;

// use Puzzle::puzzle::*;

// 7	4	3
// 2	0	5
// 8	6	1
// fn init_puzzle() -> Puzzle {
//     let Squares: Vec<Square> = vec![
//         Square {
//             actual_coordonnee: Coordonnee { x: 0, y: 0 },
//             original_coordonnee: Coordonnee { x: 0, y: 2 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 1, y: 0 },
//             original_coordonnee: Coordonnee { x: 2, y: 1 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 2, y: 0 },
//             original_coordonnee: Coordonnee { x: 2, y: 0 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 0, y: 1 },
//             original_coordonnee: Coordonnee { x: 1, y: 0 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 1, y: 1 },
//             original_coordonnee: Coordonnee { x: 1, y: 1 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 2, y: 1 },
//             original_coordonnee: Coordonnee { x: 2, y: 2 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 0, y: 2 },
//             original_coordonnee: Coordonnee { x: 0, y: 1 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 1, y: 2 },
//             original_coordonnee: Coordonnee { x: 1, y: 2 },
// 			manathan: Manathan {},
//         },
//         Square {
//             actual_coordonnee: Coordonnee { x: 2, y: 2 },
//             original_coordonnee: Coordonnee { x: 0, y: 0 },
// 			manathan: Manathan {},
//         },
//     ];
//     Puzzle::new(Squares)
// }

// fn init_puzzle() -> Puzzle {
//     let mut Squares: Vec<u32> = vec![7,4,3,2,0,5,8,6,1];
//     Puzzle::new(Squares)
// }

// fn main() {
//     let mut resolver = Resolver::new(init_puzzle());

//     dbg!(&resolver);
//     if (resolver.resolve()) {
//         println!("the puzzle is solvable");
//     } else {
//         println!("the puzzle is unsolvable");
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() == 2) {
        let puzzle: Puzzle = parser::puzzle::parse_puzzle_file(&args[1]).unwrap();
        // dbg!(puzzle);
    } else {
        println!("cargo run --release");
    }
}
