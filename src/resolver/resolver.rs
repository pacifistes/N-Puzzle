use crate::resolver::heuristic::*;
use crate::resolver::puzzle::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum Algo {
    UNIFORM_COST,
    A_STAR,
    GREEDY,
}

#[derive(Debug)]
pub struct Resolver {
    opened: BinaryHeap<RefPuzzle>,
    all_state: HashSet<RefPuzzle>,
    goal: RefPuzzle,
    algo: Algo,
    heuristics: [Option<fn(u16, u16) -> u16>; 6],
}

impl Resolver {
    pub fn new(start_state: Puzzle, goal: Puzzle) -> Resolver {
        let start_state = RefPuzzle::new(start_state);
        let mut all_state: HashSet<RefPuzzle> = HashSet::new();
        let heuristics: [Option<fn(u16, u16) -> u16>; 6] =
            [Some(manathan), None, None, None, Some(hamming), None];

        all_state.insert(start_state.clone());
        let mut opened: BinaryHeap<RefPuzzle> = BinaryHeap::new();
        opened.push(start_state);
        Resolver {
            opened,
            all_state,
            goal: RefPuzzle::new(goal),
            algo: Algo::GREEDY,
            heuristics,
        }
    }
}

impl Resolver {
    pub fn set_heuristics(&mut self, heuristics: Vec<Heuristic>) {
        for heuristic in heuristics {
            match heuristic {
                MANATHAN => self.heuristics[0] = Some(manathan),
                CHEBYSHEV => self.heuristics[1] = Some(chebyshev),
                EUCLIDIENNE => self.heuristics[2] = Some(euclidienne),
                OCTILE => self.heuristics[3] = Some(octile),
                HAMMING => self.heuristics[4] = Some(hamming),
                LINEAR_CONFLICT => self.heuristics[5] = Some(linear_conflict),
            };
        }
    }

    pub fn set_algo(&mut self, algo: Algo) {
        self.algo = algo;
    }
}

impl Resolver {
    pub fn resolve(&mut self) -> Option<Puzzle> {
        let initial_state = self.opened.peek_mut().unwrap();
        initial_state.ref_puzzle.borrow_mut().find_f(
            &self.algo,
            &self.goal.ref_puzzle.borrow(),
            &self.heuristics,
        );
        drop(initial_state);

        while !self.opened.is_empty() {
            let selected_state: RefPuzzle = self.opened.pop().unwrap();
            if selected_state == self.goal {
                self.goal = selected_state.clone();
                return Some(selected_state.ref_puzzle.borrow().clone());
            } else {
                for new_state in selected_state.ref_puzzle.borrow().expand() {
                    new_state.ref_puzzle.borrow_mut().find_f(
                        &self.algo,
                        &self.goal.ref_puzzle.borrow(),
                        &self.heuristics,
                    );
                    if !self.all_state.contains(&new_state) {
                        new_state.ref_puzzle.borrow_mut().predecessor =
                            Some(selected_state.clone());
                        self.opened.push(new_state.clone());
                        self.all_state.insert(new_state);
                    }
                }
            }
        }
        None
    }

    pub fn print(&self) {
        let mut vector: Vec<RefPuzzle> = Vec::new();
        let mut predecessor = Some(self.goal.clone());
        while (predecessor.is_some()) {
            if let Some(val) = &predecessor { vector.push(val.clone()) }
            predecessor = predecessor.unwrap().ref_puzzle.borrow().predecessor.clone();
        }
        vector.reverse();
        vector.iter().for_each(|x| x.ref_puzzle.borrow().print());
    }
}
