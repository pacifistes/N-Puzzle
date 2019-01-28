use crate::resolver::puzzle::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

fn get_size(line: &str) -> Result<usize, io::Error> {
    match line.parse::<usize>() {
        Ok(num) => Ok(num),
        Err(_) => Err(Error::new(
            ErrorKind::InvalidInput,
            "The first no-comment line should be the size of the puzzle (between 2 and 10)",
        )),
    }
}

fn add_to_state(
    mut start_state: Vec<usize>,
    line: &str,
    size: usize,
) -> Result<Vec<usize>, io::Error> {
    match line
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
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
    let mut size: usize = 0;
    let mut start_state: Vec<usize> = Vec::new();

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
    if start_state.len() != size * size {
        return Err(Error::new(ErrorKind::InvalidData, "Missing some lines"));
    }
    Ok(Puzzle::new(start_state, size, 0))
}
