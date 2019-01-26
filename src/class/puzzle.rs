// mod coordonnee;

use crate::class::coordonnee::*;
use crate::heuristic::manathan::Manathan;

#[derive(Debug, Clone)]
pub struct Square {
    pub actual_coordonnee: Coordonnee,
    pub original_coordonnee: Coordonnee,
    // pub manathan: Manathan,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    Squares: Vec<Square>,
}

impl Square {
    fn h(&self) -> u32 {
        self.manathan
            .h(&self.actual_coordonnee, &self.original_coordonnee)
    }
}

impl Puzzle {
    pub fn new(Squares: Vec<Square>) -> Puzzle {
        Puzzle { Squares }
    }

    fn h(&self) -> u32 {
        let mut total_h: u32 = 0;
        for Square in self.Squares.iter() {
            total_h += Square.h();
        }
        total_h
    }
}
