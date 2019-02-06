use crate::resolver::heuristic::*;
use crate::resolver::puzzle::*;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub enum Algo {
    UNIFORM_COST,
    A_STAR,
    GREEDY,
}

#[derive(Debug)]
pub struct Resolver {
    opened: BinaryHeap<Puzzle>,
    closed: BTreeSet<Puzzle>,
    all_state: BTreeSet<Puzzle>,
    goal: Puzzle,
    algo: Algo,
    heuristics: [Option<fn(u16, u16) -> u16>; 6],
}

impl Resolver {
    pub fn new(mut start_state: Puzzle, goal: Puzzle) -> Resolver {
        let mut all_state: BTreeSet<Puzzle> = BTreeSet::new();
        let heuristics: [Option<fn(u16, u16) -> u16>; 6] = [Some(manathan), None, None, None, None, None];
        start_state.find_h(&goal, &heuristics);
        all_state.insert(start_state.clone());
        Resolver {
            opened: BinaryHeap::from(vec![start_state.clone()]),
            closed: BTreeSet::new(),
            all_state,
            goal,
            algo : Algo::A_STAR,
            heuristics,
        }
    }
}

impl Resolver {
    pub fn set_heuristics(&mut self, heuristics : Vec<Heuristic>) {
        for heuristic in heuristics {
            match heuristic {
                MANATHAN => self.heuristics[0] = Some(manathan),
                CHEBYSHEV => self.heuristics[1] = Some(chebyshev),
                EUCLIDIENNE => self.heuristics[2] = Some(euclidienne),
                OCTILE => self.heuristics[3] = Some(octile),
                HAMMING => self.heuristics[4] = Some(hamming),
                LINEAR_CONFLICT => self.heuristics[5] = Some(linear_conflict)
            };
        }
    }

    pub fn set_algo(&mut self, algo: Algo) {
        self.algo = algo;
    }
}

impl Resolver {
    pub fn find_f(&self, state: &mut Puzzle) {
        match self.algo {
            Algo::UNIFORM_COST => state.g = 0,
            _ => state.find_h(&self.goal, &self.heuristics),
        }
    }

    pub fn resolve(&mut self) -> Option<Puzzle> {
        let mut len_closelist: usize = 0;

        while !self.opened.is_empty() {
            let selected_state: Puzzle = self.opened.pop().unwrap();
            if selected_state == self.goal {
                return Some(selected_state);
            } else {
                let index_predecessor: usize = len_closelist;
                for mut new_state in selected_state.expand() {
                    self.find_f(&mut new_state);
                    if !self.all_state.contains(&new_state) {
                        new_state.predecessor = index_predecessor;
                        self.all_state.insert(new_state.clone());
                        self.opened.push(new_state);
                    }
                }
                len_closelist += 1;
                self.closed.insert(selected_state);
            }
        }
        None
    }
}
