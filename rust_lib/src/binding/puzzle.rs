use crate::resolver::puzzle::{Puzzle, Move};
use crate::resolver::generate::r_generate_state_index;

#[repr(C)]
pub struct RVector {
    pub values: *mut u8,
    pub size: u32,
}

#[no_mangle]
pub extern "C" fn vector_free(vector: RVector) {
    if !vector.values.is_null() {
        unsafe {
            Box::from_raw(vector.values);
        }
    }
}

#[no_mangle]
pub extern "C" fn puzzle_new(state: RVector) -> *mut Puzzle {
    let size = state.size;
    unsafe {
        let state: Vec<u8> = Vec::from_raw_parts(state.values, size as usize, size as usize);
        let state_index: Vec<u8> = r_generate_state_index(&state);
        let puzzle: Puzzle = Puzzle::new(
            state,
            state_index,
            (f64::from(size)).sqrt() as u8,
            0,
            Move::NONE,
        );
        Box::into_raw(Box::new(puzzle))
    }
}

#[no_mangle]
pub extern "C" fn c_is_solvable(puzzle: *mut Puzzle, goal: *mut Puzzle) -> i8 {
    let puzzle = unsafe { (&mut *puzzle).clone() };
    let goal = unsafe { (&mut *goal) };
    puzzle.r_is_solvable(&goal) as i8
}

#[no_mangle]
pub extern "C" fn puzzle_free(puzzle: *mut Puzzle) {
    if puzzle.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(puzzle);
    }
}