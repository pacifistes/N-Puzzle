
#[derive(Debug, Clone)]
pub struct Puzzle {
    g : u32,
    h : u32,
    size: u32,
    state: Vec<u32>,
    // vect positionActual
}

impl Puzzle {
    pub fn new(state: Vec<u32>, size:u32) -> Puzzle {
        Puzzle {
            g : 0,
            h : 0,
            size,
            state,
        }
    }
}