use crate::resolver::puzzle::*;

#[derive(Debug)]
pub struct Resolver {
    opened: Vec<Puzzle>,
    closed: Vec<Puzzle>,
	goal: Puzzle
}

impl Resolver {
    pub fn new(start_state: Puzzle, goal:Puzzle) -> Resolver {
        Resolver {
            opened: vec![start_state],
            closed: Vec::new(),
            goal
        }
    }

    pub fn is_final(&self, state: &Puzzle) -> bool {
        *state == self.goal
    }
}