use crate::resolver::heuristic::*;
use crate::resolver::resolver::Algo;
use crate::resolver::generate::r_generate_state_index;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[repr(C)]
pub struct RVector {
	pub values: *mut u8,
	pub size: u32,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Move {
    TOP,
    LEFT,
    BOT,
	RIGHT,
	NONE,
}

#[no_mangle]
pub extern fn vector_free(vector: RVector) {
    if !vector.values.is_null() {
        unsafe {
			Box::from_raw(vector.values);
		}
    }
}

#[no_mangle]
pub extern fn puzzle_new(state: RVector) -> *mut Puzzle {
	let size = state.size;
	unsafe {
	    let mut state: Vec<u8> = Vec::from_raw_parts(state.values, size as usize, size as usize);
		let state_index: Vec<u8> = r_generate_state_index(&state);
	    let puzzle: Puzzle = Puzzle::new(state, state_index,  (size as f64).sqrt() as u8, 0, Move::NONE);
		Box::into_raw(Box::new(puzzle))
	}
}

#[no_mangle]
pub extern fn c_is_solvable(puzzle: *mut Puzzle, goal: *mut Puzzle) -> i8 {
	let puzzle = unsafe {
        (&mut *puzzle).clone()
    };
	let goal = unsafe {
        (&mut *goal)
    };
	puzzle.r_is_solvable(&goal) as i8
}
// impl From<(*mut u8, u8)> for RVector {
//     fn from(c_vector: (*mut u8, u8)) -> RVector {
//         RVector { values: c_vector.0, size: c_vector.1 }
//     }
// }

// impl From<RVector> for (*mut u8, u8) {
//     fn from(r_vector: RVector) -> (*mut u8, u8) {
//         (r_vector.values, r_vector.size)
//     }
// }

#[derive(Debug, Clone, Eq)]
pub struct Puzzle {
    pub g: u16,
    size: u8,
    pub state: Vec<u8>,
    pub state_index: Vec<u8>,
    pub predecessor: Option<RefPuzzle>,
	pub movement: Move,
    pub h: u16,
    pub f: u16,
}

impl Puzzle {
    pub fn new(state: Vec<u8>, state_index: Vec<u8>, size: u8, g: u16, movement: Move) -> Puzzle {
        Puzzle {
            g,
            size,
            state,
            state_index,
            predecessor: None,
			movement,
            h: 0,
            f: 0,
        }
    }
}

impl Puzzle {
    pub fn get_size(&self) -> u8 {
        self.size
    }

    pub fn get_x(&self, value: usize) -> usize {
        (value % self.size as usize)
    }

    pub fn get_y(&self, value: usize) -> usize {
        (value / self.size as usize)
    }
}

impl Puzzle {
    pub fn swap(&mut self, old_pos: usize, new_pos: usize) {
        self.state.swap(old_pos, new_pos);
    }

    pub fn r_is_solvable(&self, goal: &Puzzle) -> bool {
        let mut nbr_permute: usize = 0;
        let mut state_permute: Puzzle = self.clone();
        let actual_index = self.get_index_of_value(0) as usize;
        let goal_index = goal.get_index_of_value(0) as usize;
        let dist_x = distance(self.get_x(actual_index), goal.get_x(goal_index));
        let dist_y = distance(self.get_y(actual_index), goal.get_y(goal_index));
        let distance_empty_box: usize = manathan(dist_x, dist_y) as usize;

        for i in 0..self.state.len() {
            let value = goal.state[i];
            let actual_index: usize = state_permute.get_index_of_value(value) as usize;
            if actual_index != i {
                state_permute.swap(i, actual_index);
                nbr_permute += 1 as usize;
            }
        }
        nbr_permute % 2 == distance_empty_box % 2
    }
}

impl Puzzle {
    pub fn get_index_of_value(&self, value: u8) -> u8 {
        self.state.iter().position(|&r| r == value).unwrap() as u8
    }

    pub fn get_h_of_value(
        &self,
        value: u8,
        state_index: &Vec<u8>,
        goal_index: &Vec<u8>,
        heuristic: fn(u16, u16) -> u16,
    ) -> u16 {
        if value == 0 {
            return 0;
        }
        let actual_index: usize = state_index[(value) as usize] as usize;
        let goal_index: usize = goal_index[(value) as usize] as usize;
        let dist_x = distance(self.get_x(actual_index), self.get_x(goal_index));
        let dist_y = distance(self.get_y(actual_index), self.get_y(goal_index));
        heuristic(dist_x, dist_y)
    }

    pub fn find_h(
        &mut self,
        goal_index: &Vec<u8>,
        heuristics: &[Option<fn(u16, u16) -> u16>; 5],
        do_linear_conflict: bool,
    ) {
        self.h = heuristics
            .iter()
            .filter(|heuristic| heuristic.is_some())
            .map(|heuristic| {
                self.state
                    .iter()
                    .map(|value| {
                        self.get_h_of_value(
                            *value,
                            &self.state_index,
                            &goal_index,
                            heuristic.unwrap(),
                        )
                    })
                    .sum::<u16>()
            })
            .sum();
        if do_linear_conflict {
            self.h += linear_conflict(&self.state_index, goal_index, self.size);
        }
    }

    pub fn find_f(
        &mut self,
        algo: &Algo,
        goal_index: &Vec<u8>,
        heuristics: &[Option<fn(u16, u16) -> u16>; 5],
        do_linear_conflict: bool,
    ) {
        match algo {
            Algo::GREEDY => {
                self.find_h(goal_index, heuristics, do_linear_conflict);
                self.f = self.h;
            } // que h
            Algo::A_STAR => {
                self.find_h(goal_index, heuristics, do_linear_conflict);
                self.f = self.g + self.h;
            }
            _ => self.f = self.g,
        }
    }
}

impl Puzzle {
    pub fn apply_move(&self, old_pos: usize, new_pos: usize, movement: Move) -> RefPuzzle {
        let mut new_state: Vec<u8> = self.state.clone();
        let mut new_state_index: Vec<u8> = self.state_index.clone();
        new_state.swap(old_pos, new_pos);
        new_state_index.swap(new_state[old_pos] as usize, new_state[new_pos] as usize);
        RefPuzzle::new(Puzzle::new(
            new_state,
            new_state_index,
            self.size,
            self.g + 1,
			movement
        ))
    }

    pub fn move_left(&self, x: usize, y: usize) -> Option<RefPuzzle> {
        if x == 0 {
            return None;
        }
        Some(self.apply_move(x + y * self.size as usize, x - 1 + y * self.size as usize, Move::LEFT))
    }

    pub fn move_top(&self, x: usize, y: usize) -> Option<RefPuzzle> {
        if y == 0 {
            return None;
        }
        Some(self.apply_move(x + y * self.size as usize, x + (y - 1) * self.size as usize, Move::TOP))
    }

    pub fn move_right(&self, x: usize, y: usize) -> Option<RefPuzzle> {
        if x == (self.size - 1) as usize {
            return None;
        }
        Some(self.apply_move(x + y * self.size as usize, x + 1 + y * self.size as usize, Move::RIGHT))
    }

    pub fn move_bot(&self, x: usize, y: usize) -> Option<RefPuzzle> {
        if y == (self.size - 1) as usize {
            return None;
        }
        Some(self.apply_move(x + y * self.size as usize, x + (y + 1) * self.size as usize, Move::BOT))
    }

    pub fn expand(&self) -> Vec<RefPuzzle> {
        let blank_position: u16 = self.get_index_of_value(0) as u16;
        let x: usize = self.get_x(blank_position as usize);
        let y: usize = self.get_y(blank_position as usize);
        let expand_states: Vec<Option<RefPuzzle>> = vec![
            self.move_left(x, y),
            self.move_top(x, y),
            self.move_right(x, y),
            self.move_bot(x, y),
        ];
        expand_states.into_iter().filter_map(|x| x).collect()
    }
}

impl Puzzle {
    pub fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!(
                    "{}\t",
                    self.state[x as usize + y as usize * self.size as usize]
                );
            }
            println!("");
        }
        println!("g = {}, h = {}, f = {}", self.g, self.h, self.g + self.h);
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}

#[derive(Debug, Eq)]
pub struct RefPuzzle {
    pub ref_puzzle: Rc<RefCell<Puzzle>>,
}

impl RefPuzzle {
    pub fn new(state: Puzzle) -> RefPuzzle {
        RefPuzzle {
            ref_puzzle: Rc::new(RefCell::new(state)),
        }
    }
}

impl Clone for RefPuzzle {
    fn clone(&self) -> RefPuzzle {
        RefPuzzle {
            ref_puzzle: Rc::clone(&self.ref_puzzle),
        }
    }
}

impl PartialOrd for RefPuzzle {
    fn partial_cmp(&self, other: &RefPuzzle) -> Option<Ordering> {
        other
            .ref_puzzle
            .borrow()
            .f
            .partial_cmp(&self.ref_puzzle.borrow().f)
    }
}

impl PartialEq for RefPuzzle {
    fn eq(&self, other: &RefPuzzle) -> bool {
        self.ref_puzzle.borrow().state == other.ref_puzzle.borrow().state
    }
}

impl Ord for RefPuzzle {
    fn cmp(&self, other: &RefPuzzle) -> Ordering {
        self.ref_puzzle
            .borrow()
            .state
            .cmp(&other.ref_puzzle.borrow().state)
    }
}

impl Hash for RefPuzzle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ref_puzzle.borrow().state.hash(state);
    }
}

#[no_mangle]
pub extern fn puzzle_free(puzzle: *mut Puzzle) {
    if puzzle.is_null() { return }
    unsafe { Box::from_raw(puzzle); }
}
