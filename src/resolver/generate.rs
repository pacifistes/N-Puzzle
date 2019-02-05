use crate::resolver::puzzle::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec::Vec;

pub fn generate_sorted_puzzle(size: u8) -> Vec<u8> {
    let mut sorted: Vec<u8> = vec![0; size as usize * size as usize];
    let mut is_horizontal: bool = true;
    let mut is_increment: bool = true;
    let mut nbr_movement: u8 = 0;
    let mut movement_need: u8 = size;
    let mut x:usize = 0;
    let mut y:usize = 0;

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

pub fn generate_random_puzzle() -> Puzzle {
    let size: u8 = 3;
    let mut start_state: Vec<u8> = generate_sorted_puzzle(size);
    let mut rng = thread_rng();

    start_state.shuffle(&mut rng);
    dbg!(&start_state);
    Puzzle::new(start_state, size, 0)
}
