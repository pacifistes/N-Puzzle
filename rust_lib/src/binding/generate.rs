use crate::resolver::generate::r_generate_random_state;
use crate::resolver::generate::r_generate_sorted_state;
use crate::binding::parser::create_puzzle;
use crate::binding::parser::CreatedPuzzle;

#[no_mangle]
pub extern "C" fn c_generate_random_state(size: u32) -> CreatedPuzzle {
    create_puzzle(r_generate_random_state(size))
}

#[no_mangle]
pub extern "C" fn c_generate_sorted_state(size: u32) -> CreatedPuzzle {
    create_puzzle(r_generate_sorted_state(size))
}