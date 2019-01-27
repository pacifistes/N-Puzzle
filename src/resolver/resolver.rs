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

    pub fn resolve(&mut self) {}
    // pub fn resolve(&mut self) -> bool {
    //     let mut success: bool = false;
    //     while (!self.opened.is_empty() && success == false) {
    //         // let state: Puzzle = self.select_according_to_astar_strategy_in().clone();
    //         if (self.is_final(&state)) {
    //             success = true;
    //         }
    //         // else {
    //         //     self.closed.push(self.opened.remove(0));
    //         //     for puzzle in self.expand(&state) {
    //         //         match ()

    //         //                 if (puzzle is not in opened and not in closed) {
    //         //                     //add to open
    //         //                     // predecessor(s) <- e
    //         //                     // red g(s) <- g(e) + C(e-->s) // C(e-->s) == 1
    //         //                 }
    //         //                 else {
    //         //                     If g(s) + h(s) > g(e) + C(e-->s) + h(s)
    //         //                         // i.e f value >'potentially new' f value
    //         //                         g(s) <- g(e) + C(e-->s)
    //         //                         predecessor(s) <- e // Tu stock la reference du puzzle d'ou tu viend
    //         //                         If s is in closed
    //         //                         	closed <- closed - s
    //         //                         	opened <- opened + s
    //         //     }
    //         //     }
    //         // }
    //     }
    //     success
    // }
}
