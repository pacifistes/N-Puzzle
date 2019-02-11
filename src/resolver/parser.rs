use crate::resolver::puzzle::*;
use crate::resolver::generate::generate_state_index;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

fn get_size(line: &str) -> Result<u8, io::Error> {
    match line.parse::<u8>() {
        Ok(num) if num > 1 && num < 17 => Ok(num),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "The first no-comment line should be the size of the puzzle (between 2 and 10)",
        )),
    }
}

fn add_to_state(mut start_state: Vec<u8>, line: &str, size: u8) -> Result<Vec<u8>, io::Error> {
    match line
        .split_whitespace()
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
    {
        Ok(numbers) => {
            for number in numbers {
                match start_state.contains(&number) || number >= (size * size) as u8 {
                    true => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "The numbers must be unique and between 0 and {}",
                                size * size - 1
                            ),
                        ));
                    }
                    false => start_state.push(number),
                }
            }
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("The line must contain {} numbers", size),
            ));
        }
    }
    Ok(start_state)
}

pub fn parse(filename: &str) -> Result<Puzzle, io::Error> {
    let file = File::open(filename)?;
    let mut size: u8 = 0;
    let mut start_state: Vec<u8> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap().split('#').collect::<Vec<_>>()[0]
            .trim()
            .to_string();
        match line.is_empty() {
            true => (),
            false => match size == 0 {
                true => size = get_size(&line)?,
                false => start_state = add_to_state(start_state, &line, size)?,
            },
        }
    }
    if start_state.len() != size as usize * size as usize {
        return Err(Error::new(ErrorKind::InvalidData, "Missing some lines"));
    }
	let start_state_index: Vec<u8> = generate_state_index(&start_state);
    Ok(Puzzle::new(start_state, start_state_index, size, 0))
}
