use crate::resolver::heuristic::*;
use crate::resolver::puzzle::*;
use crate::resolver::generate::r_generate_sorted_state;
use crate::resolver::generate::r_generate_state_index;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::time::Instant;
use std::time::Duration;
use std::ptr;
use std::slice;
use libc::{c_char, uint32_t, uint8_t, size_t};
use std::ffi::CStr;
use std::ffi::CString;

#[repr(C)]
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
	pub fn new(refPuzzle: &RefPuzzle) -> RPuzzleVec {
		let mut puzzle = refPuzzle.ref_puzzle.borrow_mut().clone();
		let size = puzzle.get_size() as u32;
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
            Some(manathan)/* None*/,
            /*Some(chebyshev)*/None,
            /*Some(euclidienne)*/ None,
            None,
            /*Some(hamming)*/ None,
        ];

        all_state.insert(start_state.clone());
        let mut opened: BinaryHeap<RefPuzzle> = BinaryHeap::new();
        opened.push(start_state);
        Resolver {
            opened,
            all_state,
            goal: RefPuzzle::new(goal),
            algo: Algo::A_STAR,
            heuristics,
            do_linear_conflict: false,
        }
    }
}

#[no_mangle]
pub extern fn resolver_new(puzzle: *mut Puzzle, goal: *mut Puzzle) -> *mut Resolver {
	let puzzle = unsafe {
        (&mut *puzzle).clone()
    };
	let goal = unsafe {
        (&mut *goal).clone()
    };
	Box::into_raw(Box::new(Resolver::new(puzzle, goal)))
}

#[no_mangle]
pub extern fn c_set_heuristics(resolver: *mut Resolver, heuristics: *const Heuristic, size: size_t) {
	let heuristics = unsafe {
		slice::from_raw_parts(heuristics, size as usize).to_vec()
    };
	let resolver = unsafe {
        &mut *resolver
    };
	resolver.r_set_heuristics(heuristics);
}

#[no_mangle]
pub extern fn c_set_algo(resolver: *mut Resolver, algo: Algo) {
	let resolver = unsafe {
        &mut *resolver
    };
	resolver.r_set_algo(algo);
}

#[no_mangle]
pub extern fn c_resolve(resolver: *mut Resolver) -> ResolverInfo {
	let resolver = unsafe {
        &mut *resolver
    };
	let info = resolver.r_resolve().unwrap();
	// std::mem::forget(resolver);
	info
}

impl Resolver {
    pub fn r_set_heuristics(&mut self, heuristics: Vec<Heuristic>) {
        for heuristic in heuristics {
            match heuristic {
                MANATHAN => self.heuristics[0] = Some(manathan),
                CHEBYSHEV => self.heuristics[1] = Some(chebyshev),
                EUCLIDIENNE => self.heuristics[2] = Some(euclidienne),
                OCTILE => self.heuristics[3] = Some(octile),
                HAMMING => self.heuristics[4] = Some(hamming),
                LINEAR_CONFLICT => self.do_linear_conflict = true,
            };
        }
    }

    pub fn r_set_algo(&mut self, algo: Algo) {
        self.algo = algo;
    }
}

impl Resolver {
	pub fn get_info(&self, duration : Duration) -> ResolverInfo {
		let mut vector: Vec<RPuzzleVec> = Vec::new();
        let mut predecessor = Some(self.goal.clone());
        while (predecessor.is_some()) {
            if let Some(val) = &predecessor {
                vector.push(RPuzzleVec::new(val));
            }
            predecessor = predecessor.unwrap().ref_puzzle.borrow().predecessor.clone();
        }
        vector.reverse();
		let size = vector.len() as u32;
		let time_use = CString::new(((duration.as_secs() as f64) + (duration.subsec_nanos() as f64 / 1000_000_000.0)).to_string()).unwrap().into_raw();
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
		        let mut elapsed = start.elapsed();
				let info = Some(self.get_info(elapsed));
				// std::mem::forget(self);
                return info;
            } else {
                selected_state.ref_puzzle.borrow().expand().into_iter().for_each(|new_state| {
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
pub extern fn resolver_free(resolver: *mut Resolver) {
    if resolver.is_null() { return }
    unsafe { Box::from_raw(resolver); }
}

#[no_mangle]
pub extern fn resolve_info_free(info: ResolverInfo) {
	unsafe {
		if !info.time_use.is_null() {
			let r_string = CString::from_raw(info.time_use);
		}
		Vec::from_raw_parts(info.all_state, info.size as usize, info.size as usize).into_iter().for_each(|state| vector_free(state.state));
    }
}
