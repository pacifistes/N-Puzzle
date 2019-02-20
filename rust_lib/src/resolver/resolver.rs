use crate::resolver::heuristic::*;
use crate::resolver::heuristic::Heuristic as e_heuristic;
use crate::resolver::puzzle::*;
use libc::{c_char, size_t};
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::ffi::CString;
use std::slice;
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

#[repr(C)]
pub struct RPuzzleVec {
    pub state: RVector,
    pub movement: Move,
}

#[repr(C)]
pub struct ResolverInfo {
    all_state: *mut RPuzzleVec,
    size: u32,
    time_use: *mut c_char,
    total_state_selected: u32,
    total_state_represented: u32,
}

impl RPuzzleVec {
    pub fn new(ref_puzzle: &RefPuzzle) -> RPuzzleVec {
        let mut puzzle = ref_puzzle.ref_puzzle.borrow_mut().clone();
        let size = u32::from(puzzle.get_size());
        let movement = puzzle.movement;
        let c_values = puzzle.state.as_mut_ptr();
        std::mem::forget(puzzle.state);
        RPuzzleVec {
            state: RVector {
                values: c_values,
                size: size * size,
            },
            movement,
        }
    }
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

#[no_mangle]
pub extern "C" fn resolver_new(puzzle: *mut Puzzle, goal: *mut Puzzle) -> *mut Resolver {
    let puzzle = unsafe { (*puzzle).clone() };
    let goal = unsafe { (*goal).clone() };
    Box::into_raw(Box::new(Resolver::new(puzzle, goal)))
}

#[no_mangle]
pub extern "C" fn c_set_heuristics(
    resolver: *mut Resolver,
    heuristics: *const Heuristic,
    size: size_t,
) {
    let heuristics = unsafe { slice::from_raw_parts(heuristics, size as usize).to_vec() };
    let resolver = unsafe { &mut *resolver };
    resolver.r_set_heuristics(heuristics);
}

#[no_mangle]
pub extern "C" fn c_set_algo(resolver: *mut Resolver, algo: Algo) {
    let resolver = unsafe { &mut *resolver };
    resolver.r_set_algo(algo);
}

#[no_mangle]
pub extern "C" fn c_resolve(resolver: *mut Resolver) -> ResolverInfo {
    let resolver = unsafe {
		&mut *resolver
	};
    resolver.r_resolve().unwrap()
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

#[no_mangle]
pub extern "C" fn resolver_free(resolver: *mut Resolver) {
    if resolver.is_null() {
        return;
    }
	unsafe { Box::from_raw(resolver); }
}

#[no_mangle]
pub extern "C" fn resolve_info_free(info: ResolverInfo) {
    unsafe {
        if !info.time_use.is_null() {
            CString::from_raw(info.time_use);
        }
        Vec::from_raw_parts(info.all_state, info.size as usize, info.size as usize)
            .into_iter()
            .for_each(|state| vector_free(state.state));
    }
}
