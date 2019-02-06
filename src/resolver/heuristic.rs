use std::cmp::min;

#[warn(dead_code)]
pub enum Heuristic {
    MANATHAN,
    CHEBYSHEV,
    EUCLIDIENNE,
    OCTILE,
    HAMMING,
    LINEAR_CONFLICT
}

pub fn distance(start: usize, end: usize) -> u16 {
    (start as i16 - end as i16).abs() as u16
}

pub fn manathan(dist_x: u16, dist_y: u16) -> u16 {
    dist_x + dist_y
}

pub fn euclidienne(dist_x: u16, dist_y: u16) -> u16 {
    ((dist_x.pow(2) + dist_y.pow(2)) as f32).sqrt() as u16
}

pub fn chebyshev(dist_x: u16, dist_y: u16) -> u16 {
    ((dist_x + dist_y) as f32 - min(dist_x, dist_y) as f32) as u16
}

pub fn octile(dist_x: u16, dist_y: u16) -> u16 {
    ((dist_x + dist_y) as f32 + ((2 as f32).sqrt() - 2.0) * min(dist_x, dist_y) as f32) as u16
}

pub fn hamming(dist_x: u16, dist_y: u16) -> u16 {
    match dist_x + dist_y {
        0 => 0,
        _ => 1,
    }
}

pub fn linear_conflict(dist_x: u16, dist_y: u16) -> u16 {
    0 // modify
}

// Dominating (ensemble de plusieur heuristic)
