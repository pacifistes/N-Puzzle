use crate::resolver::puzzle::*;
use crate::resolver::heuristic::*;

#[derive(Debug)]
pub struct Resolver {
    opened: Vec<Puzzle>,
    closed: Vec<Puzzle>,
    goal: Puzzle,
    heuristic: fn(usize, usize) -> usize,
}

impl Resolver {
    pub fn new(start_state: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Resolver {
        let heuristic = match heuristic {
            Heuristic::Manathan => manathan,
            Heuristic::Chebyshev => chebyshev,
        };
        Resolver {
            opened: vec![start_state],
            closed: Vec::new(),
            goal,
            heuristic,
        }
    }

    pub fn is_final(&self, state: &Puzzle) -> bool {
        *state == self.goal
    }

    pub fn resolve(&mut self) -> Option<Puzzle> {
        let mut success: bool = false;
        while !self.opened.is_empty() && success == false {
            let (index, selected_state): (usize, Puzzle) = self.max_f();
            if self.is_final(&selected_state) {
                success = true;
                return Some(selected_state);
            }
            else {
                self.closed.push(self.opened.remove(index));
                let index_predecessor: usize = self.closed.len();
                for mut new_state in selected_state.expand() {
                    if !self.opened.contains(&new_state) && !self.closed.contains(&new_state) {
                            new_state.predecessor = index_predecessor;
                            new_state.g = selected_state.g + 1;
                            self.opened.push(new_state);
                    }
                }
            }
        }
        None
    }

    // pub fn plop(&self, new_state: Puzzle, index_predecessor: usize) {
    //     (index_old_state, old_state) : (usize, Puzzle)
    //     if (old_state.f() > new_state.f())
    //         new_state.g =
    //     // If g(s) + h(s) > g(e) + C(e-->s) + h(s)
    //     //     // i.e f value >'potentially new' f value
    //     // if new_state.f() > state.f() {
    //         // new_state.g = state.g + 1;
    //         //     predecessor(s) <- e // Tu stock la reference du puzzle d'ou tu viend
    //     if self.closed.contains(&new_state) {
    //         self.closed.remove(index_old_state));
    //         self.push(new_state);
    // }

    pub fn max_f(&self) -> (usize, Puzzle) {
        let mut final_index: usize = 0;
        let mut final_f: usize = 0;

        for (index, puzzle) in self.opened.iter().enumerate() {
            let actual_f = puzzle.f(&self.goal, &self.heuristic);
            match index == 0 || actual_f <= final_f {
                true => {
                    final_f = actual_f;
                    final_index = index;
                },
                false => (),
            }
        }
        (final_index , self.opened[final_index].clone())
    }
}
