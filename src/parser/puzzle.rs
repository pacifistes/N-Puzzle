use crate::class::{
    coordonnee::*,
    puzzle::{Puzzle, Taquin},
};
use crate::heuristic::manathan::Manathan;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

pub fn read_size_puzzle(reader : &mut BufRead) -> Result<u32, io::Error> {
    let mut line = String::new();

    while (reader.read_line(&mut line)? != 0) {
        line = line.split('#').collect::<Vec<_>>()[0].trim().to_string();
        println!("string = {}", line);
        match (line.is_empty()) {
            true => (),
            false => {
                let size: u32 = match line.parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                return Ok(size)
            }
        }
        line.clear();
    }
    Err(Error::new(ErrorKind::Other, "Invalid file : The first no-comment line should be the size of the puzzle"))
}

pub fn parse_puzzle_file(filename: &String) -> Result<Puzzle, io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let size = read_size_puzzle(&mut reader)?;
    println!("size = {}", &size);

//    for line in reader.lines() {
//         let line = line.unwrap();
//         println!("{}", line);
//     }
    let taquins: Vec<Taquin> = vec![
        Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 0 },
            original_coordonnee: Coordonnee { x: 0, y: 2 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 1, y: 0 },
            original_coordonnee: Coordonnee { x: 2, y: 1 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 2, y: 0 },
            original_coordonnee: Coordonnee { x: 2, y: 0 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 1 },
            original_coordonnee: Coordonnee { x: 1, y: 0 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 1, y: 1 },
            original_coordonnee: Coordonnee { x: 1, y: 1 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 2, y: 1 },
            original_coordonnee: Coordonnee { x: 2, y: 2 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 2 },
            original_coordonnee: Coordonnee { x: 0, y: 1 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 1, y: 2 },
            original_coordonnee: Coordonnee { x: 1, y: 2 },
			manathan: Manathan {},
        },
        Taquin {
            actual_coordonnee: Coordonnee { x: 2, y: 2 },
            original_coordonnee: Coordonnee { x: 0, y: 0 },
			manathan: Manathan {},
        },
    ];
    Ok(Puzzle::new(taquins))
}