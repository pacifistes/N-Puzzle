use crate::class::{
    coordonnee::*,
    puzzle::{Puzzle, Square},
};
use crate::heuristic::manathan::Manathan;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::fmt;

pub fn read_size(reader : &mut BufRead) -> Result<u32, io::Error> {
    let mut line = String::new();

    while (reader.read_line(&mut line)? != 0) {
        line = line.split('#').collect::<Vec<_>>()[0].trim().to_string();
        match (line.is_empty()) {
            true => (),
            false => {
                let size: u32 = match line.parse() {
                    Ok(num) => num,
                    Err(_) => break,
                };
                if size < 2 || size > 10 { return Ok(size) } else { break; }
            }
        }
        line.clear();
    }
    Err(Error::new(ErrorKind::InvalidInput, "The first no-comment line should be the size of the puzzle (between 2 and 10)"))
}

pub fn add_boxes_line(mut start_boxes: HashMap<u32, Coordonnee>, line : &String, size: &u32, y: u32) -> Result<HashMap<u32, Coordonnee>, io::Error> {
    let elems : Vec<&str> = line.split_whitespace().collect();

    if (elems.len() as u32 != *size) {
        return Err(Error::new(ErrorKind::InvalidData, format!("The lines must have {} number ", size)))
    }
    match elems.into_iter().map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, _>>() {
        Ok(numbers) => {
                for (x, number) in numbers.iter().enumerate() {
                    if (start_boxes.contains_key(number)) {
                        return Err(Error::new(ErrorKind::InvalidData, format!("The numbers must be unique and between 0 and {}", size * size - 1)))
                    }
                    else {
                        start_boxes.insert(*number, Coordonnee::new(y, x));
                    }
                }
        },
            //Verifier si les duplicate
            //Creer les new u32,Coordonnee
            //Les ajoutes a la liste
        _ => return Err(Error::new(ErrorKind::InvalidData, format!("The numbers must be unique and between 0 and {}", size * size - 1)))
    }
    Ok(start_boxes)
}

// Key = position in 1D
// Value = position in 2D (y,x)
pub fn read_boxes(reader: &mut BufRead, size: &u32) -> Result<HashMap<u32, Coordonnee>, io::Error> {
    let mut line = String::new();
    let mut start_boxes: HashMap<u32, Coordonnee> = HashMap::new();
    let mut nbr_valid_line:u32 = 0;
    let mut y: u32 = 0;

    while (reader.read_line(&mut line)? != 0) {
        line = line.split('#').collect::<Vec<_>>()[0].trim().to_string();
        match (line.is_empty()) {
            true => (),
            false => {
                start_boxes = add_boxes_line(start_boxes, &line, size, &y)?;
                nbr_valid_line += 1;
                if (nbr_valid_line == *size) {
                    return Ok(start_boxes);
                }
                y += 1;
            }
        }
        line.clear();
    }
    Err(Error::new(ErrorKind::Other, "Invalid file : missing line(s) about position of boxes in the puzzle"))
}

pub fn parse_puzzle_file(filename: &String) -> Result<Puzzle, io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let size = read_size(&mut reader)?;
    let start_boxes = read_boxes(&mut reader, &size)?;
    dbg!(start_boxes);
    // let goal_boxes = goal_boxes(&size);
    // let boxes = boxes(start_boxes, goal_boxes);
    // let puzzle: Puzzle::new(size, boxes);
    // Ok(puzzle)
    Err(Error::new(ErrorKind::Other, "tmp error"))
}


// Exemple
// 1	2	3 	4
// 12	13	14	5
// 11	0	15	6
// 10	9	8	7
// *******************************************************************
// HashMap (KEY = position, value = (coordonnee)) original_position
// HashMap (KEY = position, value) = (Coordonnee))
// HashMap(Key = f ou h, value = Puzzle)
// *******************************************************************

// nbr_movement = 0
// movement_need = size - 1
// movement = Horizontal
// direction = increment
// while (index < size * size)
// {
// 	if (index == 0)
// 		continue
// 	List.push({x, y});
// 	if (movement == Horizontal)
// 		x = (increment) ? +1 ; -1
// 	else
// 		y = (increment) ? + 1 : - 1
// 	nbr_movement++;

// 	if (nbr_movement ==  movement_need) {
// 		movement = (Horizontal) ? vertical : Horizontal
// 		if (vertical) {
// 			movement_need--;
// 		}
// 		if (Horizontal) {
// 			direction = (increment) ? decrement : increment;
// 		}
// 		nbr_movement = 0;
// 	}
// }
// *******************************************************************


// Returns true if `start` belongs to the same permutation group as `goal`
// template <uint size>
// bool isSolvable(const Puzzle<size> & start, const Puzzle<size> & goal) {
//     auto startInversions = inversions(start);
//     auto goalInversions = inversions(goal);

//     if (size % 2 == 0) { // In this case, the row of the '0' tile matters
//         startInversions += start.indexOf(0) / size;
//         goalInversions += goal.indexOf(0) / size;
//     }

//     return (startInversions % 2 == goalInversions % 2);
// }

// *******************************************************************
// La formule dit:
// 	Si la largeur de la grille est impair, le nombre d'inversions dans une situation résoluble est pair.
// 	Si la largeur de la grille est paire et que le blanc est sur une ligne paire à compter du bas (avant-dernière, quatrième avant, etc.), le nombre d'inversions dans une situation pouvant être résolue est impair.
// 	Si la largeur de la grille est paire et que le blanc se trouve sur une ligne impaire comptant à partir du bas (dernier, dernier dernier, cinquième dernier, etc.), le nombre d'inversions dans une situation résoluble est pair.

// Cela nous donne cette formule pour déterminer la solvabilité:
// ((largeur de la grille impaire) && (#inversions pair)) || ((largeur de la grille même) && ((blanc sur la ligne impaire du bas) == (#inversions même)))

// *******************************************************************
// The formula says:
// If the grid width is odd, then the number of inversions in a solvable situation is even.
// If the grid width is even, and the blank is on an even row counting from the bottom (second-last, fourth-last etc), then the number of inversions in a solvable situation is odd.
// If the grid width is even, and the blank is on an odd row counting from the bottom (last, third-last, fifth-last etc) then the number of inversions in a solvable situation is even.
// That gives us this formula for determining solvability:

// ( (grid width odd) && (#inversions even) )  ||  ((grid width even) && ((blank on odd row from bottom) == (#inversions even)) )

