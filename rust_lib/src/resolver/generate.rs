use rand::seq::SliceRandom;
use std::vec::Vec;
use std::io;
use std::io::{Error, ErrorKind};

pub fn r_generate_sorted_state(size: u32) -> Result<Vec<u8>, io::Error> {
    if size < 2 || size > 15 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "The first no-comment line should be the size of the puzzle (between 2 and 15)",
        ))
    }
    let mut sorted: Vec<u8> = vec![0; size as usize * size as usize];
    let mut is_horizontal: bool = true;
    let mut is_increment: bool = true;
    let mut nbr_movement: u32 = 0;
    let mut movement_need: u32 = size;
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
    Ok(sorted)
}

pub fn r_generate_state_index(state: &[u8]) -> Vec<u8> {
    let range: Vec<u8> = (0..state.len() as u8).collect();
    range
        .iter()
        .map(|value| state.iter().position(|&r| r == *value).unwrap() as u8)
        .collect()
}

pub fn r_generate_random_state(size: u32) -> Result<Vec<u8>, io::Error> {
    let mut start_state: Vec<u8> = r_generate_sorted_state(size)?;
    start_state.shuffle(&mut rand::thread_rng());
    Ok(start_state)
}
