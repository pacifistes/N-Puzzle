use crate::class::{
    coordonnee::*,
    puzzle::{Puzzle, Square},
};

#[derive(Debug, Clone)]
pub struct Manathan {}

impl Manathan {
    pub fn h(&self, a: &Coordonnee, b: &Coordonnee) -> u32 {
        ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as u32
    }
}
