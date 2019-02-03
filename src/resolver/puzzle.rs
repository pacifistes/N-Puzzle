use crate::resolver::heuristic::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub struct Puzzle {
    pub g: usize,
    size: usize,
    pub state: Vec<usize>,
    pub predecessor: usize,
    pub h: usize,
}

impl Puzzle {
    pub fn new(state: Vec<usize>, size: usize, g: usize) -> Puzzle {
        Puzzle {
            g,
            size,
            state,
            predecessor: 0,
            h: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_x(&self, value: usize) -> usize {
        value % self.size
    }

    pub fn get_y(&self, value: usize) -> usize {
        value / self.size
    }
}

impl Puzzle {
    pub fn is_solvable(&self, goal: &Puzzle) -> bool {
        let mut nbr_permute: usize = 0;
        let mut state_permute: Puzzle = self.clone();
        let actual_index = self.get_index_of_value(0);
        let goal_index = goal.get_index_of_value(0);
        let dist_x = distance(self.get_x(actual_index), goal.get_x(goal_index));
        let dist_y = distance(self.get_y(actual_index), goal.get_y(goal_index));
        let distance_empty_box: usize = manathan(dist_x, dist_y);

        for i in 0..self.state.len() {
            let value = goal.state[i];
            let actual_index: usize = state_permute.get_index_of_value(value);
            if actual_index != i {
                state_permute.swap(i, actual_index);
                nbr_permute += 1 as usize;
            }
        }
        nbr_permute % 2 == distance_empty_box % 2
    }

    pub fn swap(&mut self, old_pos: usize, new_pos: usize) {
        self.state.swap(old_pos, new_pos);
    }

    pub fn get_index_of_value(&self, value: usize) -> usize {
        self.state.iter().position(|&r| r == value).unwrap()
    }

    pub fn print(&self) {
        for y in 0..self.size {
            for x in 0..self.size {
                print!("{}\t", self.state[x + y * self.size]);
            }
            println!("");
        }
        println!("g = {}, h = {}, f = {}", self.g, self.h, self.g + self.h);
    }
}

/*
**
*/
impl Puzzle {

    pub fn apply_move(&self, old_pos: usize, new_pos: usize) -> Puzzle {
        let mut new_state: Vec<usize> = self.state.clone();
        new_state.swap(old_pos, new_pos);
        Puzzle::new(new_state, self.size, self.g + 1)
    }

    pub fn move_left(&self, x: usize, y: usize) -> Option<Puzzle> {
        match x == 0 {
            true => None,
            false => Some(self.apply_move(x + y * self.size, x - 1 + y * self.size)),
        }
    }

    pub fn move_top(&self, x: usize, y: usize) -> Option<Puzzle> {
        match y == 0 {
            true => None,
            false => Some(self.apply_move(x + y * self.size, x + (y - 1) * self.size)),
        }
    }

    pub fn move_right(&self, x: usize, y: usize) -> Option<Puzzle> {
        match x == self.size - 1 {
            true => None,
            false => Some(self.apply_move(x + y * self.size, x + 1 + y * self.size)),
        }
    }

    pub fn move_bot(&self, x: usize, y: usize) -> Option<Puzzle> {
        match y == self.size - 1 {
            true => None,
            false => Some(self.apply_move(x + y * self.size, x + (y + 1) * self.size)),
        }
    }

    pub fn expand(&self) -> Vec<Puzzle> {
        let blank_position: usize = self.get_index_of_value(0) as usize;
        let x: usize = self.get_x(blank_position);
        let y: usize = self.get_y(blank_position);
        let expand_states: Vec<Option<Puzzle>> = vec![
            self.move_left(x, y),
            self.move_top(x, y),
            self.move_right(x, y),
            self.move_bot(x, y),
        ];
        expand_states.into_iter().filter_map(|x| x).collect()
    }

    pub fn f(&self) -> usize {
        self.g + self.h
    }

	pub fn get_h_of_value(&self, value: usize, goal: &Puzzle, heuristic: fn(usize, usize) -> usize) -> usize {
		let actual_index = self.get_index_of_value(value);
		let goal_index = goal.get_index_of_value(value);
		let dist_x = distance(self.get_x(actual_index), goal.get_x(goal_index));
		let dist_y = distance(self.get_y(actual_index), goal.get_y(goal_index));
		heuristic(dist_x, dist_y)
	}

    pub fn init_h(&mut self, goal: &Puzzle, heuristic: fn(usize, usize) -> usize) {
		let mut h: usize = 0;
		for i in 0..self.state.len() {
            h += self.get_h_of_value(i, goal, heuristic);
        }
		self.h = h;
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Puzzle) -> Ordering {
		// print!("cmp|");
        self.state.cmp(&other.state)
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
		// print!("eq|");
        self.state == other.state
    }
}

// impl PartialOrd for Puzzle {
//     fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
// 		// print!("partial_cmp|");
//         self.f().partial_cmp(&other.f())
//     }
// }

//For binaryHeap
impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
		// print!("partial_cmp|");
        other.f().partial_cmp(&self.f())
    }
}

impl Hash for Puzzle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}