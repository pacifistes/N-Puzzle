use std::i32;

#[derive(Debug, Clone)]
pub struct Coordonnee {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
pub struct Taquin {
    pub actual_coordonnee: Coordonnee,
    pub original_coordonnee: Coordonnee,
    pub manathan: Manathan,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    taquins: Vec<Taquin>,
}

#[derive(Debug)]
pub struct Resolver {
    opened: Vec<Puzzle>,
    closed: Vec<Puzzle>,
    // success: bool,
}

#[derive(Debug, Clone)]
pub struct Manathan {}


impl Manathan {
    pub fn h(&self, a: &Coordonnee, b: &Coordonnee) -> u32 {
        ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as u32
    }
}

impl Taquin {
    fn h(&self) -> u32 {
        self.manathan.h(&self.actual_coordonnee, &self.original_coordonnee)
    }
}


impl Puzzle {
    pub fn new(taquins: Vec<Taquin>) -> Puzzle {
        Puzzle { taquins }
    }

    fn h(&self) -> u32 {
        let mut total_h: u32 = 0;
        for taquin in self.taquins.iter() {
            total_h += taquin.h();
        }
        total_h
    }
}

impl Resolver {
    pub fn new(puzzle: Puzzle) -> Resolver {
        Resolver {
            opened: vec![puzzle],
            closed: Vec::new(),
            // success: false,
        }
    }

    fn select_according_to_astar_strategy_in(&self) -> &Puzzle {
        &self.opened[0]
    }

    pub fn resolve(&mut self) -> bool {
        let mut success: bool = false;
        while (!self.opened.is_empty() && success == false) {
            let state: Puzzle = self.select_according_to_astar_strategy_in().clone();
            if (self.is_final(&state)) {
                success = true;
            }
            else {
                // dbg!(&self.opened[0].taquins[0]);
                // self.opened[0].taquins[0].actual_coordonnee.x = 1250;
                // dbg!(&self.opened[0].taquins[0]);
                // let puzzle = self.opened.remove(0);

                self.closed.push(self.opened.remove(0));
                // dbg!(&self);
                for puzzle in self.expand(&state) {
                    dbg!(&puzzle);
        //             if (puzzle is not in opened and not in closed) {
        //                 //add to open
        //                 // predecessor(s) <- e
        //                 // red g(s) <- g(e) + C(e-->s) // C(e-->s) == 1
        //             }
        //             else {
        //                 If g(s) + h(s) > g(e) + C(e-->s) + h(s)
        //                     // i.e f value >'potentially new' f value
        //                     g(s) <- g(e) + C(e-->s)
        //                     predecessor(s) <- e // Tu stock la reference du puzzle d'ou tu viend
        //                     If s is in closed
        //                     	closed <- closed - s
        //                     	opened <- opened + s
                    // }
                }
            }
        //     // dbg!(&state.taquins[0]);
        //     // dbg!(&state.taquins[0]);
            break;
        }
        success
    }

    fn is_final(&self, puzzle: &Puzzle) -> bool {
        return false;
    }

    fn expand(&self, puzzle: &Puzzle) -> Vec<Puzzle> {
        vec![Puzzle::new(vec![Taquin {
            actual_coordonnee: Coordonnee { x: 0, y: 0 },
            original_coordonnee: Coordonnee { x: 0, y: 2 },
            manathan: Manathan{},
        }])]
    }
}

/*
    Taquin

    Puzzle

    Resolver
*/
