use std::cmp::min;

#[warn(dead_code)]
pub enum Heuristic {
    Manathan,
    Chebyshev,
}

pub fn distance(start: usize, end: usize) -> u16 {
    (start as i16 - end as i16).abs() as u16
}

pub fn manathan(dist_x: u16, dist_y: u16) -> u16 {
    dist_x as u16 + dist_y as u16
}

// pub fn euclidienne(dist_x: u16, dist_y: u16) -> u16 {
//     ((dist_x.pow(2) + dist_y.pow(2)) as isize).sqrt()
// }

pub fn chebyshev(dist_x: u16, dist_y: u16) -> u16 {
    (dist_x + dist_y) - min(dist_x, dist_y)
}

// pub fn octile(dist_x: u16, dist_y: u16) -> u16 {
//     (dist_x + dist_y) + ((2 as f64).sqrt() - 2) * min(dist_x, dist_y)
// }
