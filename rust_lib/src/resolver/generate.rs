use crate::resolver::puzzle::*;
use rand::seq::SliceRandom;
use std::vec::Vec;

pub fn r_generate_sorted_state(size: u8) -> Vec<u8> {
    let mut sorted: Vec<u8> = vec![0; size as usize * size as usize];
    let mut is_horizontal: bool = true;
    let mut is_increment: bool = true;
    let mut nbr_movement: u8 = 0;
    let mut movement_need: u8 = size;
    let mut x: usize = 0;
    let mut y: usize = 0;

    for index in 1..(size * size) {
        sorted[x + y * size as usize] = index as u8;
        nbr_movement += 1;
        if nbr_movement == movement_need {
            is_horizontal = !is_horizontal;
            if is_horizontal {
                is_increment = !is_increment;
            } else {
                movement_need -= 1;
            }
            nbr_movement = 0;
        }
        if is_horizontal {
            x = if is_increment { x + 1 } else { x - 1 };
        } else {
            y = if is_increment { y + 1 } else { y - 1 };
        }
    }
    sorted
}

pub fn r_generate_state_index(state: &[u8]) -> Vec<u8> {
    let range: Vec<u8> = (0..state.len() as u8).collect();
    range
        .iter()
        .map(|value| state.iter().position(|&r| r == *value).unwrap() as u8)
        .collect()
}

pub fn r_generate_random_state() -> Vec<u8> {
    let size: u8 = 3;
    let mut start_state: Vec<u8> = r_generate_sorted_state(size);

    start_state.shuffle(&mut rand::thread_rng());
    start_state
}

#[no_mangle]
pub extern "C" fn c_generate_random_state() -> RVector {
    let mut r_values = r_generate_random_state();
    let size = r_values.len() as u32;
    let c_values = r_values.as_mut_ptr();

    std::mem::forget(r_values);
    RVector {
        values: c_values,
        size,
    }
}

#[no_mangle]
pub extern "C" fn c_generate_sorted_state(size: u32) -> RVector {
    let mut r_values = r_generate_sorted_state(size as u8);
    let size = r_values.len() as u32;
    let c_values = r_values.as_mut_ptr();

    std::mem::forget(r_values);
    RVector {
        values: c_values,
        size,
    }
}
