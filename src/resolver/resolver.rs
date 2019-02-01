use crate::resolver::heuristic::*;
use crate::resolver::puzzle::*;
use std::time::Instant;

#[derive(Debug)]
pub struct Resolver {
    opened: Vec<Puzzle>,
    closed: Vec<Puzzle>,
    goal: Puzzle,
    heuristic: fn(usize, usize) -> usize,
}

impl Resolver {
    pub fn new(mut start_state: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Resolver {
        let heuristic = match heuristic {
            Heuristic::Manathan => manathan,
            Heuristic::Chebyshev => chebyshev,
        };
        start_state.init_h(&goal, heuristic);
        Resolver {
            opened: vec![start_state],
            closed: Vec::new(),
            goal,
            heuristic,
        }
    }

    pub fn is_final(&self, state: &Puzzle) -> bool {
        *state == self.goal
    }

    pub fn resolve(&mut self) -> Option<Puzzle> {
        let mut len_closelist: usize = 0;
        while !self.opened.is_empty() {
            let start = Instant::now();
            let (index, selected_state): (usize, Puzzle) = self.max_f();
            let elapsed = start.elapsed();
            println!("Time find f : {:?}", elapsed);
            if self.is_final(&selected_state) {
                return Some(selected_state);
            } else {
                self.closed.push(self.opened.remove(index));
                let index_predecessor: usize = len_closelist;
                for mut new_state in selected_state.expand() {
                    new_state.init_h(&self.goal, self.heuristic);
                    if !self.opened.contains(&new_state) && !self.closed.contains(&new_state) {
                        new_state.predecessor = index_predecessor;
                        new_state.g = selected_state.g + 1;
                        self.opened.push(new_state);
                    }
                }
                len_closelist += 1;
            }
        }
        None
    }

    pub fn max_f(&self) -> (usize, Puzzle) {
        let mut final_index: usize = 0;
        let mut final_f: usize = 0;

        for (index, puzzle) in self.opened.iter().enumerate() {
            let actual_f = puzzle.f();
            match index == 0 || actual_f <= final_f {
                true => {
                    final_f = actual_f;
                    final_index = index;
                }
                false => (),
            }
        }
        (final_index, self.opened[final_index].clone())
    }

    // pub fn max_f(&self) -> (usize, Puzzle) {

    // }
}

// use crate::resolver::heuristic::*;
// use crate::resolver::puzzle::*;
// use std::collections::HashMap;

// #[derive(Debug)]
// pub struct Resolver {
//     opened: HashMap<Puzzle, usize>,
//     closed: Vec<Puzzle>,
//     goal: Puzzle,
//     heuristic: fn(usize, usize) -> usize,
// }

// impl Resolver {
//     pub fn new(mut start_state: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Resolver {
//         let heuristic = match heuristic {
//             Heuristic::Manathan => manathan,
//             Heuristic::Chebyshev => chebyshev,
//         };
//         start_state.init_h(&goal, heuristic);
//         let opened: <Puzzle, usize> = HashMap::new();
//         opened.insert(start_state, start_state.f());
//         Resolver {
//             opened: vec![start_state],
//             closed: Vec::new(),
//             goal,
//             heuristic,
//         }
//     }

//     pub fn is_final(&self, state: &Puzzle) -> bool {
//         *state == self.goal
//     }

//     pub fn resolve(&mut self) -> Option<Puzzle> {
//         let mut len_closelist: usize = 0;
//         while !self.opened.is_empty() {
//             let (index, selected_state): (usize, Puzzle) = self.max_f();
//             if self.is_final(&selected_state) {
//                 return Some(selected_state);
//             } else {
//                 self.closed.push(self.opened.remove(index));
//                 let index_predecessor: usize = len_closelist;
//                 for mut new_state in selected_state.expand() {
//                     new_state.init_h(&self.goal, self.heuristic);
//                     if !self.opened.contains(&new_state) && !self.closed.contains(&new_state) {
//                         new_state.predecessor = index_predecessor;
//                         new_state.g = selected_state.g + 1;
//                         self.opened.push(new_state);
//                     }
//                 }
//                 len_closelist += 1;
//             }
//         }
//         None
//     }

//     pub fn max_f(&self) -> (usize, Puzzle) {
//         let mut final_index: usize = 0;
//         let mut final_f: usize = 0;

//         for (index, puzzle) in self.opened.iter().enumerate() {
//             let actual_f = puzzle.f();
//             match index == 0 || actual_f <= final_f {
//                 true => {
//                     final_f = actual_f;
//                     final_index = index;
//                 }
//                 false => (),
//             }
//         }
//         (final_index, self.opened[final_index].clone())
//     }
// }