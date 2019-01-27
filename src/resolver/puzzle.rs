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

    pub fn is_solvable(&self) -> bool {
        true
    }
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}
