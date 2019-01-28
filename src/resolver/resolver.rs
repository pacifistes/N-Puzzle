use crate::resolver::puzzle::*;

#[derive(Debug)]
pub struct Resolver {
    opened: Vec<Puzzle>,
    closed: Vec<Puzzle>,
    goal: Puzzle,
}

impl Resolver {
    pub fn new(start_state: Puzzle, goal: Puzzle) -> Resolver {
        Resolver {
            opened: vec![start_state],
            closed: Vec::new(),
            goal,
        }
    }

    pub fn is_final(&self, state: &Puzzle) -> bool {
        *state == self.goal
    }

    // pub fn resolve(&mut self) {}
    pub fn resolve(&mut self) -> Option<Puzzle> {
        let mut success: bool = false;
        while !self.opened.is_empty() && success == false {
            let state: Puzzle = self.opened.iter().max().unwrap().clone();
            if self.is_final(&state) {
                success = true;
                return Some(state);
            }
            else {
                self.closed.push(self.opened.remove(0));
                for mut new_state in state.expand() {
                    match !self.opened.contains(&new_state) && !self.closed.contains(&new_state) {
                        true => {
                            // predecessor(s) <- e
                            new_state.g = state.g + 1;
                            self.opened.push(new_state);
                        },
                        false => {
                            // If g(s) + h(s) > g(e) + C(e-->s) + h(s)
                            //     // i.e f value >'potentially new' f value
                            if new_state.f() > state.f() {
                                new_state.g = state.g + 1;
                                //     predecessor(s) <- e // Tu stock la reference du puzzle d'ou tu viend
                                if self.closed.contains(&new_state) {
                                        //         closed <- closed - s
                                        //         opened <- opened + s
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }
}
