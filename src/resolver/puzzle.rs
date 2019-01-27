#[derive(Debug, Clone)]
pub struct Puzzle {
    g: u32,
    h: u32,
    size: u32,
    state: Vec<u32>,
    // vect positionActual
}

impl Puzzle {
    pub fn new(state: Vec<u32>, size: u32) -> Puzzle {
        Puzzle {
            g: 0,
            h: 0,
            size,
            state,
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn inversion(&self) -> u32 {
        let mut nbr_permut: u32 = 0;
        let mut state_permute: Vec<u32> = self.state.clone();
        let mut tmp: u32 = 0;
        //let index = self.state.iter().position(|&r| r == 0).unwrap();
        for i in 0..self.state.len() {
            let mut value = self.state[i];
    //        println!("value{}", &value);
            let index:i32 = state_permute.iter().position(|&r| r == value).unwrap() as i32;
            if (value == 0) {
                value = self.state.len() as u32;
            }
            let permute:u32 = (index - value as i32 + 1).abs() as u32;
            // println!("{:?}", &state_permute);
            // println!("index {}, value{}", &index, &value);
            // println!("the {} give us {} permutation", &value, &permute);
            state_permute.remove(index as usize);
            if (value == self.state.len() as u32) {
                state_permute.insert((value - 1) as usize, tmp);
            }
            else {
                state_permute.insert((value - 1) as usize, value);
            }
            nbr_permut += permute;
        }
        if (self.size % 2 == 0) {
            nbr_permut += self.state.iter().position(|&r| r == 0).unwrap() as u32;
        }
        nbr_permut
    }

    pub fn is_solvable(&self, goal: &Puzzle) -> bool {
        let mut start_inversion: u32 = self.inversion();
        let mut goal_inversion: u32 = goal.inversion();

        start_inversion % 2 == goal_inversion % 2
    }
}

impl Puzzle {
    pub fn move_left(&self) -> Option<Puzzle> {
        None
    }

    pub fn move_top(&self) -> Option<Puzzle> {
        None
    }

    pub fn move_right(&self) -> Option<Puzzle> {
        None
    }

    pub fn move_bot(&self) -> Option<Puzzle> {
        None
    }

    pub fn expand(&self) -> Vec<Puzzle> {
        let expand_states: Vec<Option<Puzzle>> = vec![
            self.move_left(),
            self.move_top(),
            self.move_right(),
            self.move_bot(),
        ];

        expand_states.into_iter().filter_map(|x| x).collect()
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}
