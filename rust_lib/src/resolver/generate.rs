#![crate_type = "staticlib"]
// extern crate rand;
use crate::resolver::puzzle::*;
use std::vec::Vec;
use std::slice;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn r_generate_sorted_puzzle(size: u8) -> Vec<u8> {
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

pub fn r_generate_state_index(state: &Vec<u8>) -> Vec<u8> {
    let range: Vec<u8> = (0..state.len() as u8).collect();
    range
        .iter()
        .map(|value| state.iter().position(|&r| r == *value).unwrap() as u8)
        .collect()
}

pub fn r_generate_random_puzzle() -> Vec<u8> {
    let size: u8 = 3;
    let mut start_state: Vec<u8> = r_generate_sorted_puzzle(size);
    // let mut rng = thread_rng();

    // start_state.shuffle(&mut rng); //TODO A FAIRE MARCHER
	start_state
}

#[no_mangle]
pub extern fn c_generate_random_puzzle() -> RVector {
	let mut c_values = r_generate_random_puzzle();
	let r_values = c_values.as_mut_ptr();

	std::mem::forget(c_values);
	RVector  {
		values: r_values,
		size: 3,
	}
}

#[no_mangle]
pub extern fn c_generate_sorted_puzzle(size: u8) -> RVector {
	let mut c_values = r_generate_sorted_puzzle(size);
	let r_values = c_values.as_mut_ptr();

	std::mem::forget(c_values);
	RVector {
		values: r_values,
		size,
	}
}