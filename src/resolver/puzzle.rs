use crate::resolver::heuristic::*;

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub g: usize,
    size: usize,
    state: Vec<usize>,
    pub predecessor: usize,
    h: usize,
}

impl Puzzle {
    pub fn new(state: Vec<usize>, size: usize, g: usize) -> Puzzle {
        Puzzle {
            g,
            size,
            state,
            predecessor: 0,
            h:0
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
        let distance_empty_box: usize =
            manathan(goal.get_index_of_value(0), self.get_index_of_value(0));

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

    pub fn init_h(&mut self, goal: &Puzzle, heuristic: &fn(usize, usize) -> usize) {
        let mut h = 0;

        for i in 0..self.state.len() {
            let actual_index = self.get_index_of_value(i);
            let goal_index = goal.get_index_of_value(i);
            let dist_x = distance(self.get_x(actual_index), goal.get_x(goal_index));
            let dist_y = distance(self.get_y(actual_index), goal.get_y(goal_index));
            h += heuristic(dist_x, dist_y);
        }
        self.h = h;
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}
