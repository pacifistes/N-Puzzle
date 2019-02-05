use crate::resolver::heuristic::*;
use crate::resolver::puzzle::*;
use std::collections::BinaryHeap;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct Resolver {
	opened: BinaryHeap<Puzzle>,
	closed: BTreeSet<Puzzle>,
	goal: Puzzle,
	heuristic: fn(u16, u16) -> u16,
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
			closed: BTreeSet::new(),
			goal,
			heuristic,
		}
	}

	pub fn resolve(&mut self) -> Option<Puzzle> {
		let mut len_closelist: usize = 0;

		while !self.opened.is_empty() {
			let selected_state: Puzzle = self.opened.pop().unwrap();
			if selected_state == self.goal {
				return Some(selected_state);
			} else {
				let index_predecessor: usize = len_closelist;
				for mut new_state in selected_state.expand() {
					new_state.init_h(&self.goal, self.heuristic);
					if !self.closed.contains(&new_state) && self.opened.iter().position(|r| r == &new_state).is_none() {
						new_state.predecessor = index_predecessor;
						self.opened.push(new_state);
					}
				}
				len_closelist += 1;
				self.closed.insert(selected_state);
			}
		}
		None
	}
}
