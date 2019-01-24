// mod coordonnee;

use super::coordonnee::*;
use crate::heuristic::manathan::Manathan;

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

impl Taquin {
    fn h(&self) -> u32 {
        self.manathan
            .h(&self.actual_coordonnee, &self.original_coordonnee)
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
