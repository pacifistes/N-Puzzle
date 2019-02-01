7# N-Puzzle

Realiser un solver de N-Puzzle (Square)


Algorithmes : A*
snail Square
Square from 1 to N


Heuristique : must be minimum 3 heuristique
Manhattan-distance is mandatory


/**********************************\


You have to manage both randomly determined states (of your own generation of
course), or input files that specify a starting board, the format of which is described
in the appendix.

• The cost associated with each transition is always 1.

• The user must be able to choose between at LEAST 3 (relevant) heuristic functions.
The Manhattan-distance heuristic is mandatory, the other two are up to you. By
"relevant" we mean they must be admissible (Read up on what this means) and
they must be something other than "just return a random value because #YOLO".



At the end of the search, the program has to provide the following values:
	Total number of states ever selected in the "opened" set (complexity in time)
	Maximum number of states ever represented in memory at the same time
		during the search (complexity in size)
	Number of moves required to transition from the initial state to the final state,
		according to the search
	The ordered sequence of states that make up the solution, according to the
		search
	The puzzle may be unsolvable, in which case you have to inform the user and
		exit

• Configure the appropriate g(x) and h(x) functions to run both the uniform-cost
and greedy searches. Execute with the same output (Of course, the solution may
be different. Read up on why, that’s the point.)



Pseudo-Implementation of A*
***************************************************
set opened <- { initial }
			// States to be examined and candidates to expansion

set closed <- NULL
			// States already selected by the algorithm, compared to the solution and expanded
bool succes <- false
while (opened != NULLand SUCCES == FALSE)
	state e -< select_according_to_Astar_strategy_in (opened)
	If is_final (e) // Compares e to a solution state
		Then
			success <- true
		Else
			opened <- opened -e
			closed <- closed + e
			ForEach state s in expand(e) do
				If (s is not in opened and s is not in closed)
					opened <- opened + s
					predecessor(s) <- e
					g(s) <- g(e) + C(e-->s)
				Else // s is in opened or in closed
					If g(s) + h(s) > g(e) + C(e-->s) + h(s)
					// i.e f value >'potentially new' f value
						g(s) <- g(e) + C(e-->s)
						predecessor(s) <- e
						If s is in closed
							closed <- closed - s
							opened <- opened + s




***************************************************

*****************************************************
TODO
	Parser
		- ignorer les lignes commentaires
		- premiere ligne le nombre de case de cote du puzzle
		- les autres ligne peuvent avoir des commentaire

	A*


	Implementation des heuristique


	Preparation des Information
		Nombre d'etat total selectionne in the Opened (complexity in time)
		Nombre maximum d'etat representer dans la memoire au meme moment (Complexity in size)
		Nombre de move
		The ordered sequence of states that make up the solution, according to the search
		The puzzle may be unsolvable, in which case you have to inform the user and
		exit


7	4	3
2	0	5
8	6	1

Step 1
2	2	0
2	2	1
1	2	4
total h = 16 g = 0


Object:

size 3
	coordonnee :

Puzzle (Position)

***********************

Step 2

7	0	3
2	4	5
8	6	1


total h = g = 1

------------------------------
7	4	3
2	5	0
8	6	1
total h = 17 g  = 1

-------------------------------

7	4	3
0	2	5
8	6	1
total h = 14 g  = 1

-----------------------------

7	4	3
2	6	5
8	0	1
total h = 18 g  = 1


****************************

Puzzle
	List<Square> Squares
	<!-- int		G; -->

	<!-- int		getH(); -->
	<!-- int		F() { G + this.getH()} -->

Square
	Coordonnees actualCoordonne;
	Coordonnees finalCoordonnee;

	int			getH();


Coordonnee
	int	x;
	int	y;;

Resolver:
	List<Puzzle>	opened // original State To put
	List<Puzzle>	closed

	bool		resolve()
	void		init() {
		Pour chaque Square initialisation de depart
	}

main ()
{
	Puzzle puzzle = parseFile();

	if (puzzle == Null)
		print('error in file')

	Resolver resolver = new Resolver(Puzzle);
	resolver.init()
	if (resolver.resolve())
		print('le probleme est resolu')
	else
		print(le puzzle est insolvable)

}













7	4	3
2	0	5
8	6	1
[7,4,3,2,0,5,8,6,1] Pour 0  [ pos = 4 | x = 4 % size = 1 | y = 4 / size = 1]
					Pour 1  [ pos = 8 | x = 8 % size = 2 | y = 8 / size = 2]
					Pour 2
					Pour 3
					Pour 4  [pos = 1 | x = 1 % size = 1 | y = 1 / size = 0]
					Pour 5  [pos = 5 | x = 5 % size = 2 | y = 5 / size = 1]
					Pour 6
					Pour 7
					Pour 8

1	2	3
8	0	4
7	6	5
[1,2,3,8,0,4,7,6,5] Pour 0  [ pos = 4 | x = 4 % size = 1 | y = 4 / size = 1]
					Pour 1  [ pos = 0 | x = 0 % size = 0 | y = 0 / size = 0]
					Pour 2
					Pour 3
					Pour 4  [pos = 5 | x = 5 % size = 2 | y = 5 / size = 1]
					Pour 5
					Pour 6
					Pour 7
					Pour 8

Puzzle () {
	G : u32
	h : u32
	Vect positionActual
	h(VecGoal)
}

Resolver () {
	Vect Goal[] original
	Vect<Puzzle> open
	Vect<Puzzle> close

}











****************************


// use crate::class::{
//     puzzle::{Puzzle, Square},
// };
// use crate::heuristic::manathan::Manathan;
// use std::i32;

// #[derive(Debug)]
// pub struct Resolver {
//     opened: Vec<Puzzle>,
//     closed: Vec<Puzzle>,
//     // success: bool,
// }

// impl Resolver {
//     pub fn new(puzzle: Puzzle) -> Resolver {
//         Resolver {
//             opened: vec![puzzle],
//             closed: Vec::new(),
//             // success: false,
//         }
//     }

//     fn select_according_to_astar_strategy_in(&self) -> &Puzzle {
//         &self.opened[0]
//     }

//     pub fn resolve(&mut self) -> bool {
//         let mut success: bool = false;
//         while (!self.opened.is_empty() && success == false) {
//             let state: Puzzle = self.select_according_to_astar_strategy_in().clone();
//             if (self.is_final(&state)) {
//                 success = true;
//             } else {
//                 // dbg!(&self.opened[0].Squares[0]);
//                 // self.opened[0].Squares[0].actual_coordonnee.x = 1250;
//                 // dbg!(&self.opened[0].Squares[0]);
//                 // let puzzle = self.opened.remove(0);

//                 self.closed.push(self.opened.remove(0));
//                 // dbg!(&self);
//                 for puzzle in self.expand(&state) {
//                     dbg!(&puzzle);
//                     //             if (puzzle is not in opened and not in closed) {
//                     //                 //add to open
//                     //                 // predecessor(s) <- e
//                     //                 // red g(s) <- g(e) + C(e-->s) // C(e-->s) == 1
//                     //             }
//                     //             else {
//                     //                 If g(s) + h(s) > g(e) + C(e-->s) + h(s)
//                     //                     // i.e f value >'potentially new' f value
//                     //                     g(s) <- g(e) + C(e-->s)
//                     //                     predecessor(s) <- e // Tu stock la reference du puzzle d'ou tu viend
//                     //                     If s is in closed
//                     //                     	closed <- closed - s
//                     //                     	opened <- opened + s
//                     // }
//                 }
//             }
//             //     // dbg!(&state.Squares[0]);
//             //     // dbg!(&state.Squares[0]);
//             break;
//         }
//         success
//     }

//     fn is_final(&self, puzzle: &Puzzle) -> bool {
//         return false;
//     }

//     fn expand(&self, puzzle: &Puzzle) -> Vec<Puzzle> {
//         vec![Puzzle::new(vec![Square {
//             actual_coordonnee: Coordonnee { x: 0, y: 0 },
//             original_coordonnee: Coordonnee { x: 0, y: 2 },
//             manathan: Manathan {},
//         }])]
//     }
// }




// mod coordonnee;

// use crate::class::coordonnee::*;
// use crate::heuristic::manathan::Manathan;

#[derive(Debug, Clone)]
pub struct Square {
    // pub actual_coordonnee: Coordonnee,
    // pub original_coordonnee: Coordonnee,
    // pub manathan: Manathan,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    // G : u32
    // h : u32
    // Vect positionActual
}

impl Square {
    // fn h(&self) -> u32 {
    //     self.manathan
    //         .h(&self.actual_coordonnee, &self.original_coordonnee)
    // }
}

impl Puzzle {
    // pub fn new(Squares: Vec<Square>) -> Puzzle {
    //     Puzzle { Squares }
    // }

    // fn h(&self) -> u32 {
    //     let mut total_h: u32 = 0;
    //     for Square in self.Squares.iter() {
    //         total_h += Square.h();
    //     }
    //     total_h
    // }
}





use crate::class::{
    puzzle::{Puzzle, Square},
};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::fmt;

// pub fn read_size(reader : &mut BufRead) -> Result<u32, io::Error> {
//     let mut line = String::new();

//     while (reader.read_line(&mut line)? != 0) {
//         line = line.split('#').collect::<Vec<_>>()[0].trim().to_string();
//         match (line.is_empty()) {
//             true => (),
//             false => {
//                 let size: u32 = match line.parse() {
//                     Ok(num) => num,
//                     Err(_) => break,
//                 };
//                 if size < 2 || size > 10 { return Ok(size) } else { break; }
//             }
//         }
//         line.clear();
//     }
//     Err(Error::new(ErrorKind::InvalidInput, "The first no-comment line should be the size of the puzzle (between 2 and 10)"))
// }

// pub fn add_boxes_line(mut start_boxes: HashMap<u32, Coordonnee>, line : &String, size: &u32, y: u32) -> Result<HashMap<u32, Coordonnee>, io::Error> {
//     let elems : Vec<&str> = line.split_whitespace().collect();

//     if (elems.len() as u32 != *size) {
//         return Err(Error::new(ErrorKind::InvalidData, format!("The lines must have {} number ", size)))
//     }
//     match elems.into_iter().map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, _>>() {
//         Ok(numbers) => {
//                 for (x, number) in numbers.iter().enumerate() {
//                     if (start_boxes.contains_key(number)) {
//                         return Err(Error::new(ErrorKind::InvalidData, format!("The numbers must be unique and between 0 and {}", size * size - 1)))
//                     }
//                     else {
//                         start_boxes.insert(*number, Coordonnee::new(y, x));
//                     }
//                 }
//         },
//             //Verifier si les duplicate
//             //Creer les new u32,Coordonnee
//             //Les ajoutes a la liste
//         _ => return Err(Error::new(ErrorKind::InvalidData, format!("The numbers must be unique and between 0 and {}", size * size - 1)))
//     }
//     Ok(start_boxes)
// }

// Key = position in 1D
// Value = position in 2D (y,x)
// pub fn read_boxes(reader: &mut BufRead, size: &u32) -> Result<HashMap<u32, Coordonnee>, io::Error> {
//     let mut line = String::new();
//     let mut start_boxes: HashMap<u32, Coordonnee> = HashMap::new();
//     let mut nbr_valid_line:u32 = 0;
//     let mut y: u32 = 0;

//     while (reader.read_line(&mut line)? != 0) {
//         line = line.split('#').collect::<Vec<_>>()[0].trim().to_string();
//         match (line.is_empty()) {
//             true => (),
//             false => {
//                 start_boxes = add_boxes_line(start_boxes, &line, size, &y)?;
//                 nbr_valid_line += 1;
//                 if (nbr_valid_line == *size) {
//                     return Ok(start_boxes);
//                 }
//                 y += 1;
//             }
//         }
//         line.clear();
//     }
//     Err(Error::new(ErrorKind::Other, "Invalid file : missing line(s) about position of boxes in the puzzle"))
// }

pub fn parse_puzzle_file(filename: &String) -> Result<Puzzle, io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    // let puzzle: Puzzle =
    // let goal Vec::<u32> = generat_goal(puzzle);
    // let size = read_size(&mut reader)?;
    // let start_boxes = read_boxes(&mut reader, &size)?;
    // dbg!(start_boxes);
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


