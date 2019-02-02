/*{
    use crate::resolver::heuristic::*;
    use crate::resolver::puzzle::*;
    use std::collections::BinaryHeap;
    use std::time::Instant;

    #[derive(Debug)]
    pub struct Resolver {
        opened: BinaryHeap<Puzzle>,
        closed: Vec<Puzzle>,
        goal: Puzzle,
        heuristic: fn(usize, usize) -> usize,
    }

    impl Resolver {
        pub fn new(mut start_state: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Resolver {
            let heuristic = match heuristic {
                Heuristic::Manathan => manathan,
                Heuristic::Chebyshev => chebyshev,
            };
            start_state.init_h(&goal, heuristic);
            Resolver {
                opened: BinaryHeap::from(vec![start_state]),
                closed: Vec::new(),
                goal,
                heuristic,
            }
        }

        pub fn is_final(&self, state: &Puzzle) -> bool {
            *state == self.goal
        }

        pub fn resolve(&mut self) -> Option<Puzzle> {
            let mut len_closelist: usize = 0;
            while !self.opened.is_empty() {
                let selected_state: Puzzle = self.opened.pop().unwrap();
                selected_state.print();
                if self.is_final(&selected_state) {
                    return Some(selected_state);
                } else {
                    let index_predecessor: usize = len_closelist;
                    for mut new_state in selected_state.expand() {
                        new_state.init_h(&self.goal, self.heuristic);
                            let start = Instant::now();
                        // if !self.closed.contains(&new_state) {
                        // if !self.opened.into_vec().contains(&new_state) && !self.closed.contains(&new_state) {
                            new_state.predecessor = index_predecessor;
                            self.opened.push(new_state);
                            let elapsed = start.elapsed();
                            println!("Time find f : {:?}", elapsed);
                        //}
                    }
                    len_closelist += 1;
                    if (len_closelist > 3) {
                        return None
                    }
                    self.closed.push(selected_state);
                }
            }
            None
        }

        pub fn min_f(&self) -> usize {
            let mut final_index: usize = 0;
            let mut final_f: usize = 0;
            for (index, puzzle) in self.opened.iter().enumerate() {
                let actual_f = puzzle.f();
                match index == 0 || actual_f <= final_f {
                    true => {
                        final_f = actual_f;
                        final_index = index;
                    }
                    false => (),
                }
            }
            final_index
        }
    }
}*/

//HashMap
/*
{
    // use crate::resolver::heuristic::*;
    // use crate::resolver::puzzle::*;
    // use std::collections::HashMap;
    // use std::time::Instant;


    // #[derive(Debug)]
    // pub struct Resolver {
    //     opened: HashMap<Puzzle, usize>,
    //     closed: Vec<Puzzle>,
    //     goal: Puzzle,
    //     heuristic: fn(usize, usize) -> usize,
    // }

    // impl Resolver {
    //     pub fn new(mut start_state: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Resolver {
    //         let heuristic = match heuristic {
    //             Heuristic::Manathan => manathan,
    //             Heuristic::Chebyshev => chebyshev,
    //         };
    //         start_state.init_h(&goal, heuristic);
    //         let mut opened: HashMap<Puzzle, usize> = HashMap::new();
    //         let f:usize = start_state.f();
    //         opened.insert(start_state, f);
    //         Resolver {
    //             opened,
    //             closed: Vec::new(),
    //             goal,
    //             heuristic,
    //         }
    //     }

    //     pub fn is_final(&self, state: &Puzzle) -> bool {
    //         *state == self.goal
    //     }

    //     pub fn resolve(&mut self) -> Option<Puzzle> {
    //         let mut len_closelist: usize = 0;
    //         while !self.opened.is_empty() {
    //             let start = Instant::now();
    //             let selected_state: Puzzle = self.opened.iter().min().unwrap().0.clone();
    //             self.opened.remove(&selected_state);
    //             if self.is_final(&selected_state) {
    //                 return Some(selected_state);
    //             } else {
    //                 let elapsed = start.elapsed();
    //                 println!("Time find f : {:?}", elapsed);
    //                 let index_predecessor: usize = len_closelist;
    //                 for mut new_state in selected_state.expand() {
    //                     new_state.init_h(&self.goal, self.heuristic);
    //                     if !self.opened.contains_key(&new_state) && !self.closed.contains(&new_state) {
    //                         new_state.predecessor = index_predecessor;
    //                         let f:usize = new_state.f();
    //                         self.opened.insert(new_state, f);
    //                     }
    //                 }
    //                 self.closed.push(selected_state);
    //                 len_closelist += 1;
    //             }
    //         }
    //         None
    //     }
    // }
}
*/

//Vector
/*
{*/
    use crate::resolver::heuristic::*;
    use crate::resolver::puzzle::*;
    use std::time::Instant;

    #[derive(Debug)]
    pub struct Resolver {
        opened: Vec<Puzzle>,
        closed: Vec<Puzzle>,
        goal: Puzzle,
        heuristic: fn(usize, usize) -> usize,
    }

    impl Resolver {
        pub fn new(mut start_state: Puzzle, goal: Puzzle, heuristic: Heuristic) -> Resolver {
            let heuristic = match heuristic {
                Heuristic::Manathan => manathan,
                Heuristic::Chebyshev => chebyshev,
            };
            start_state.init_h(&goal, heuristic);
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
            let mut len_closelist: usize = 0;
            while !self.opened.is_empty() {
                let selected_state: Puzzle = self.opened.remove(self.min_f());
                selected_state.print();
                if self.is_final(&selected_state) {
                    return Some(selected_state);
                } else {
                    let index_predecessor: usize = len_closelist;
                    for mut new_state in selected_state.expand() {
                        new_state.init_h(&self.goal, self.heuristic);
                        let start = Instant::now();
                        if !self.opened.contains(&new_state) && !self.closed.contains(&new_state) {
                            new_state.predecessor = index_predecessor;
                            self.opened.push(new_state);
                        }
                        let elapsed = start.elapsed();
                        println!("Time find f : {:?}", elapsed);
                    }
                    len_closelist += 1;
                    if (len_closelist == 5) {
                        return None;
                    }
                    self.closed.push(selected_state);
                }
            }
            None
        }

        pub fn min_f(&self) -> usize {
            let mut final_index: usize = 0;
            let mut final_f: usize = 0;
            for (index, puzzle) in self.opened.iter().enumerate() {
                let actual_f = puzzle.f();
                match index == 0 || actual_f <= final_f {
                    true => {
                        final_f = actual_f;
                        final_index = index;
                    }
                    false => (),
                }
            }
            final_index
        }
    }
/*}
*/
