use crate::resolver::heuristic::*;
use crate::resolver::heuristic::Heuristic as e_heuristic;
use crate::resolver::puzzle::*;
use crate::binding::resolver::ResolverInfo;
use crate::binding::resolver::RPuzzleVec;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::ffi::CString;
use std::time::Duration;
use std::time::Instant;

#[repr(C)]
#[derive(Debug)]
pub enum Algo {
    UniformCost,
    AStar,
    Greedy,
}

#[derive(Debug)]
pub struct Resolver {
    opened: BinaryHeap<RefPuzzle>,
    all_state: HashSet<RefPuzzle>,
    goal: RefPuzzle,
    algo: Algo,
    heuristics: [Option<fn(u16, u16) -> u16>; 5],
    do_linear_conflict: bool,
}

impl Resolver {
    pub fn new(start_state: Puzzle, goal: Puzzle) -> Resolver {
        let start_state = RefPuzzle::new(start_state);
        let mut all_state: HashSet<RefPuzzle> = HashSet::new();
        let heuristics: [Option<fn(u16, u16) -> u16>; 5] = [
            Some(manathan),
            None,
            None,
            None,
            None,
        ];

        all_state.insert(start_state.clone());
        let mut opened: BinaryHeap<RefPuzzle> = BinaryHeap::new();
        opened.push(start_state);
        Resolver {
            opened,
            all_state,
            goal: RefPuzzle::new(goal),
            algo: Algo::AStar,
            heuristics,
            do_linear_conflict: false,
        }
    }
}

impl Resolver {
    pub fn r_set_heuristics(&mut self, heuristics: Vec<Heuristic>) {
        for heuristic in heuristics {
            match heuristic {
                e_heuristic::Manathan => self.heuristics[0] = Some(manathan),
                e_heuristic::Chebyshev => self.heuristics[1] = Some(chebyshev),
                e_heuristic::Euclidienne => self.heuristics[2] = Some(euclidienne),
                e_heuristic::Octile => self.heuristics[3] = Some(octile),
                e_heuristic::Hamming => self.heuristics[4] = Some(hamming),
                e_heuristic::LinearConflict => self.do_linear_conflict = true,
            };
        }
    }

    pub fn r_set_algo(&mut self, algo: Algo) {
        self.algo = algo;
    }
}

impl Resolver {
    pub fn get_info(&self, duration: Duration) -> ResolverInfo {
        let mut vector: Vec<RPuzzleVec> = Vec::new();
        let mut predecessor = Some(self.goal.clone());
        while predecessor.is_some() {
            if let Some(val) = &predecessor {
                vector.push(RPuzzleVec::new(val));
            }
            predecessor = predecessor.unwrap().ref_puzzle.borrow().predecessor.clone();
        }
        vector.reverse();
        let size = vector.len() as u32;
        let time_use = CString::new(
            ((duration.as_secs() as f64) + (f64::from(duration.subsec_nanos()) / 1_000_000_000.0))
                .to_string(),
        )
        .unwrap()
        .into_raw();
        let c_vector = vector.as_mut_ptr();
        std::mem::forget(vector);
        ResolverInfo {
            all_state: c_vector,
            size,
            time_use,
            total_state_selected: size,
            total_state_represented: self.all_state.len() as u32,
        }
    }

    pub fn r_resolve(&mut self) -> Option<ResolverInfo> {
        let start = Instant::now();
        let initial_state = self.opened.peek_mut().unwrap();
        initial_state.ref_puzzle.borrow_mut().find_f(
            &self.algo,
            &self.goal.ref_puzzle.borrow().state_index,
            &self.heuristics,
            self.do_linear_conflict,
        );
        drop(initial_state);

        while !self.opened.is_empty() {
            let selected_state: RefPuzzle = self.opened.pop().unwrap();
            if selected_state == self.goal {
                self.goal = selected_state.clone();
                let info = Some(self.get_info(start.elapsed()));
                return info;
            } else {
                selected_state
                    .ref_puzzle
                    .borrow()
                    .expand()
                    .into_iter()
                    .for_each(|new_state| {
                        new_state.ref_puzzle.borrow_mut().find_f(
                            &self.algo,
                            &self.goal.ref_puzzle.borrow().state_index,
                            &self.heuristics,
                            self.do_linear_conflict,
                        );
                        if !self.all_state.contains(&new_state) {
                            new_state.ref_puzzle.borrow_mut().predecessor =
                                Some(selected_state.clone());
                            self.opened.push(new_state.clone());
                            self.all_state.insert(new_state);
                        }
                    });
            }
        }
        None
    }
}
