use crate::resolver::heuristic::Heuristic;
use crate::resolver::resolver::{Algo, Resolver};
use crate::binding::puzzle::{RVector, vector_free};
use crate::resolver::puzzle::{Puzzle, RefPuzzle, Move};
use libc::{size_t, c_char};
use std::slice;
use std::ffi::CString;

#[repr(C)]
pub struct RPuzzleVec {
    pub state: RVector,
    pub movement: Move,
}

#[repr(C)]
pub struct ResolverInfo {
    pub all_state: *mut RPuzzleVec,
    pub size: u32,
    pub time_use: *mut c_char,
    pub total_state_selected: u32,
    pub total_state_represented: u32,
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
            .for_each(|c_puzzle| vector_free(c_puzzle.state));
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