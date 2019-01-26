use crate::resolver::puzzle::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

fn get_size(line: &String) -> Result<u32, io::Error> {
    match line.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(Error::new(
            ErrorKind::InvalidInput,
            "The first no-comment line should be the size of the puzzle (between 2 and 10)",
        )),
    }
}

fn add_to_state(
    mut start_state: Vec<u32>,
    line: &String,
    size: &u32,
) -> Result<Vec<u32>, io::Error> {
    match line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()
    {
        Ok(numbers) => {
            for number in numbers {
                match start_state.contains(&number) || number >= size * size {
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

pub fn parse(filename: &String) -> Result<Puzzle, io::Error> {
    let file = File::open(filename)?;
    let mut size: u32 = 0;
    let mut start_state: Vec<u32> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap().split('#').collect::<Vec<_>>()[0]
            .trim()
            .to_string();
        match line.is_empty() {
            true => (),
            false => match size == 0 {
                true => size = get_size(&line)?,
                false => start_state = add_to_state(start_state, &line, &size)?,
            },
        }
    }
    if start_state.len() as u32 != size * size {
        return Err(Error::new(ErrorKind::InvalidData, "Missing some lines"));
    }
    Ok(Puzzle::new(start_state, size))
}
