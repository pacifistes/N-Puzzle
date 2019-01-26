use crate::resolver::puzzle::*;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::vec::Vec;

pub fn generate_sorted_puzzle(size: &u32) -> Vec<u32> {
    let mut sorted: Vec<u32> = vec![0; (size * size) as usize];
    let mut is_horizontal: bool = true;
    let mut is_increment: bool = true;
    let mut nbr_movement: u32 = 0;
    let mut movement_need: u32 = *size;
    let mut x = 0;
    let mut y = 0;

    for index in 1..(size * size) {
        sorted[x + y * (*size) as usize] = index;
        nbr_movement += 1;
        if nbr_movement == movement_need {
            is_horizontal = !is_horizontal;
            match is_horizontal {
                true => {is_increment = !is_increment;},
                false => {movement_need -= 1;}
            }
            nbr_movement = 0;
        }
        match is_horizontal {
            true => {
                x = if is_increment {x + 1} else {x - 1};
            },
            false => {
                y = if is_increment {y + 1} else {y - 1};
            }
        }
    }
    sorted
}

pub fn generate_random_puzzle() -> Puzzle {

    let size : u32 = 4;
    let mut start_state: Vec<u32> = generate_sorted_puzzle(&size);
    let mut rng = thread_rng();

    start_state.shuffle(&mut rng);
    dbg!(&start_state);
    Puzzle::new(start_state, size)
}