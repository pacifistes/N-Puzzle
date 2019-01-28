#[derive(Debug, Clone)]
pub struct Puzzle {
    g: u32,
    h: u32,
    size: u32,
    pub state: Vec<u32>,
    // vect positionActual
}

impl Puzzle {
    pub fn new(state: Vec<u32>, size: u32, g: u32) -> Puzzle {
        Puzzle {
            g,
            h: 0,
            size,
            state,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_x(&self, value: &u32) -> u32 {
        value % self.size
    }

    pub fn get_y(&self, value: &u32) -> u32 {
        value / self.size
    }
}

// Problem soluble si la parite de la permutation est identique a la parite de la case vide
impl Puzzle {
    pub fn manathan(&self, pos: &u32, pos2: &u32) -> usize {
        ((self.get_x(pos) as i32 - self.get_x(pos2) as i32).abs() - (self.get_y(pos) as i32- self.get_y(pos2)as i32).abs()) as usize
    }

    pub fn is_solvable(&self, goal: &Puzzle) -> bool {
        let mut nbr_permute: usize = 0;
        let mut state_permute: Vec<u32> = self.state.clone();
        let lenght: usize = self.state.len();
        let mut test: usize = 0;

        for i in 0..lenght {
            let mut value = goal.state[i];
            let actual_index: usize = state_permute.iter().position(|&r| r == value).unwrap() as usize;
            // state_permute.remove(actual_index);
            // state_permute.insert(i, value);
//            nbr_permute += (actual_index as i32 - i as i32).abs() as usize;
            if (value == 0 || self.state[actual_index] == 0) {
                test = self.manathan(&(i as u32), &(actual_index as u32));
            }
            if (actual_index != i) {
                state_permute.swap(i, actual_index);
                nbr_permute += 1 as usize;
            }
        }
        println!("nbr_permute = {} | {}", nbr_permute, test);
        nbr_permute % 2 == test % 2
    }
}


// impl Puzzle {
//     pub fn inversion(&self) -> u32 {
//         let mut nbr_permut: u32 = 0;
//         let mut state_permute: Vec<u32> = self.state.clone();
//         let mut tmp: u32 = 0;
//         //let index = self.state.iter().position(|&r| r == 0).unwrap();
//         for i in 0..self.state.len() {
//             let mut value = self.state[i];
//             //        println!("value{}", &value);
//             let index: i32 = state_permute.iter().position(|&r| r == value).unwrap() as i32;
//             if (value == 0) {
//                 value = self.state.len() as u32;
//             }
//             let permute: u32 = (index - value as i32 + 1).abs() as u32;
//             // println!("{:?}", &state_permute);
//             // println!("index {}, value{}", &index, &value);
//             // println!("the {} give us {} permutation", &value, &permute);
//             state_permute.remove(index as usize);
//             if (value == self.state.len() as u32) {
//                 state_permute.insert((value - 1) as usize, tmp);
//             } else {
//                 state_permute.insert((value - 1) as usize, value);
//             }
//             nbr_permut += permute;
//         }
//         if self.size % 2 == 0 {
//             nbr_permut += self.state.iter().position(|&r| r == 0).unwrap() as u32;
//         }
//         nbr_permut
//     }

//     pub fn is_solvable(&self, goal: &Puzzle) -> bool {
//         let mut start_inversion: u32 = self.inversion();
//         let mut goal_inversion: u32 = goal.inversion();

//         start_inversion % 2 == goal_inversion % 2
//     }
// }

impl Puzzle {
    pub fn move_left(&self, x: u32, y: u32) -> Option<Puzzle> {
        match x == 0 {
            true => None,
            false => {
                let mut new_state: Vec<u32> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x - 1 + y * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn move_top(&self, x: u32, y: u32) -> Option<Puzzle> {
        match y == 0 {
            true => None,
            false => {
                let mut new_state: Vec<u32> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x + (y - 1) * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn move_right(&self, x: u32, y: u32) -> Option<Puzzle> {
        match x == self.size - 1 {
            true => None,
            false => {
                let mut new_state: Vec<u32> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x + 1 + y * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn move_bot(&self, x: u32, y: u32) -> Option<Puzzle> {
        match y == self.size - 1 {
            true => None,
            false => {
                let mut new_state: Vec<u32> = self.state.clone();
                let old_pos: usize = (x + y * self.size) as usize;
                let new_pos: usize = (x + (y + 1) * self.size) as usize;
                new_state.swap(old_pos, new_pos);
                Some(Puzzle::new(new_state, self.size, self.g + 1))
            }
        }
    }

    pub fn expand(&self) -> Vec<Puzzle> {
        let blank_position: u32 = self.state.iter().position(|&r| r == 0).unwrap() as u32;
        let x: u32 = self.get_x(&blank_position);
        let y: u32 = self.get_y(&blank_position);
        let expand_states: Vec<Option<Puzzle>> = vec![
            self.move_left(x, y),
            self.move_top(x, y),
            self.move_right(x, y),
            self.move_bot(x, y),
        ];

        expand_states.into_iter().filter_map(|x| x).collect()
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}
