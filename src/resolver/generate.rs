use crate::resolver::puzzle::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec::Vec;

pub fn generate_sorted_puzzle(size: usize) -> Vec<usize> {
    let mut sorted: Vec<usize> = vec![0; (size * size) as usize];
    let mut is_horizontal: bool = true;
    let mut is_increment: bool = true;
    let mut nbr_movement: usize = 0;
    let mut movement_need: usize = size;
    let mut x = 0;
    let mut y = 0;

    for index in 1..(size * size) {
        sorted[x + y * size] = index;
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
    let size: usize = 3;
    let mut start_state: Vec<usize> = generate_sorted_puzzle(size);
    let mut rng = thread_rng();

    start_state.shuffle(&mut rng);
    dbg!(&start_state);
    Puzzle::new(start_state, size, 0)
}
