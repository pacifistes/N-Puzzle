use crate::class::{
    coordonnee::*,
    puzzle::{Puzzle, Square},
};
use crate::heuristic::manathan::Manathan;
use std::i32;

#[derive(Debug)]
pub struct Resolver {
    opened: Vec<Puzzle>,
    closed: Vec<Puzzle>,
    // success: bool,
}

impl Resolver {
    pub fn new(puzzle: Puzzle) -> Resolver {
        Resolver {
            opened: vec![puzzle],
            closed: Vec::new(),
            // success: false,
        }
    }

    fn select_according_to_astar_strategy_in(&self) -> &Puzzle {
        &self.opened[0]
    }

    pub fn resolve(&mut self) -> bool {
        let mut success: bool = false;
        while (!self.opened.is_empty() && success == false) {
            let state: Puzzle = self.select_according_to_astar_strategy_in().clone();
            if (self.is_final(&state)) {
                success = true;
            } else {
                // dbg!(&self.opened[0].Squares[0]);
                // self.opened[0].Squares[0].actual_coordonnee.x = 1250;
                // dbg!(&self.opened[0].Squares[0]);
                // let puzzle = self.opened.remove(0);

                self.closed.push(self.opened.remove(0));
                // dbg!(&self);
                for puzzle in self.expand(&state) {
                    dbg!(&puzzle);
                    //             if (puzzle is not in opened and not in closed) {
                    //                 //add to open
                    //                 // predecessor(s) <- e
                    //                 // red g(s) <- g(e) + C(e-->s) // C(e-->s) == 1
                    //             }
                    //             else {
                    //                 If g(s) + h(s) > g(e) + C(e-->s) + h(s)
                    //                     // i.e f value >'potentially new' f value
                    //                     g(s) <- g(e) + C(e-->s)
                    //                     predecessor(s) <- e // Tu stock la reference du puzzle d'ou tu viend
                    //                     If s is in closed
                    //                     	closed <- closed - s
                    //                     	opened <- opened + s
                    // }
                }
            }
            //     // dbg!(&state.Squares[0]);
            //     // dbg!(&state.Squares[0]);
            break;
        }
        success
    }

    fn is_final(&self, puzzle: &Puzzle) -> bool {
        return false;
    }

    fn expand(&self, puzzle: &Puzzle) -> Vec<Puzzle> {
        vec![Puzzle::new(vec![Square {
            actual_coordonnee: Coordonnee { x: 0, y: 0 },
            original_coordonnee: Coordonnee { x: 0, y: 2 },
            manathan: Manathan {},
        }])]
    }
}
