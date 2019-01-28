use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub struct Puzzle {
    pub g: usize,
    h_each_square: &fn(&self, ),
    size: usize,
    state: Vec<usize>,
}

impl Puzzle {
    pub fn new(state: Vec<usize>, size: usize, g: usize) -> Puzzle {
        Puzzle {
            g,
            h: 0,
            size,
            state,
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

    pub fn f(&self) -> usize {
        self.g + self.h();
    }
}

impl Puzzle {
    pub fn manathan(&self, actual_pos: usize, goal_pos: usize) -> usize {
        ((self.get_x(actual_pos) as isize - self.get_x(goal_pos) as isize).abs()
            + (self.get_y(actual_pos) as isize - self.get_y(goal_pos) as isize).abs())
            as usize
    }

    pub fn is_solvable(&self, goal: &Puzzle) -> bool {
        let mut nbr_permute: usize = 0;
        let mut state_permute: Puzzle = self.clone();
        let test: usize = self.manathan(goal.get_index_of_value(0), self.get_index_of_value(0));

        for i in 0..self.state.len() {
            let value = goal.state[i];
            let actual_index: usize = state_permute.get_index_of_value(value);
            if actual_index != i {
                state_permute.swap(i, actual_index);
                nbr_permute += 1 as usize;
            }
        }
        println!("nbr_permute = {} | {}", nbr_permute, test);
        nbr_permute % 2 == test % 2
    }

    pub fn swap(&mut self, old_pos: usize, new_pos: usize) {
        self.state.swap(old_pos, new_pos);
    }

    pub fn get_index_of_value(&self, value: usize) -> usize {
        self.state.iter().position(|&r| r == value).unwrap()
    }
}

impl Puzzle {
    pub fn move_left(&self, x: usize, y: usize) -> Option<Puzzle> {
        match x == 0 {
            true => None,
            false => {
                let mut new_state: Vec<usize> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x - 1 + y * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn move_top(&self, x: usize, y: usize) -> Option<Puzzle> {
        match y == 0 {
            true => None,
            false => {
                let mut new_state: Vec<usize> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x + (y - 1) * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn move_right(&self, x: usize, y: usize) -> Option<Puzzle> {
        match x == self.size - 1 {
            true => None,
            false => {
                let mut new_state: Vec<usize> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x + 1 + y * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn move_bot(&self, x: usize, y: usize) -> Option<Puzzle> {
        match y == self.size - 1 {
            true => None,
            false => {
                let mut new_state: Vec<usize> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x + (y + 1) * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn expand(&self) -> Vec<Puzzle> {
        let blank_position: usize = self.state.get_index_of_value(0) as usize;
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
}


impl Ord for Puzzle {
    fn cmp(&self, other: &Puzzle) -> Ordering {
        self.h.cmp(&other.h)
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Puzzle) -> Option<Ordering> {
        self.h.partial_cmp(&other.h)
    }
}