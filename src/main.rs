mod resolver;

use resolver::puzzle::{Coordonnee, Puzzle, Resolver, Taquin, Manathan};
// use Puzzle::puzzle::*;


// 7	4	3
// 2	0	5
// 8	6	1
fn init_puzzle() -> Puzzle {
    let taquins: Vec<Taquin> = vec![
        Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 0 },
            original_coordonnee: Coordonnee { x: 0, y: 2 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 1, y: 0 },
            original_coordonnee: Coordonnee { x: 2, y: 1 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 2, y: 0 },
            original_coordonnee: Coordonnee { x: 2, y: 0 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 1 },
            original_coordonnee: Coordonnee { x: 1, y: 0 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 1, y: 1 },
            original_coordonnee: Coordonnee { x: 1, y: 1 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 2, y: 1 },
            original_coordonnee: Coordonnee { x: 2, y: 2 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 2 },
            original_coordonnee: Coordonnee { x: 0, y: 1 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 1, y: 2 },
            original_coordonnee: Coordonnee { x: 1, y: 2 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 2, y: 2 },
            original_coordonnee: Coordonnee { x: 0, y: 0 },
			manathan: Manathan {},
        },
    ];
    Puzzle::new(taquins)
}


// fn init_puzzle() -> Puzzle {
//     let mut taquins: Vec<u32> = vec![7,4,3,2,0,5,8,6,1];
//     Puzzle::new(taquins)
// }

fn main() {
    let mut resolver = Resolver::new(init_puzzle());

    dbg!(&resolver);
    if (resolver.resolve()) {
        println!("the puzzle is solvable");
    } else {
        println!("the puzzle is unsolvable");
    }
}
