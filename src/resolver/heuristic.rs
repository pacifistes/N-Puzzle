use std::cmp::min;

#[warn(dead_code)]
pub enum Heuristic {
    Manathan,
    Chebyshev,
}

pub fn distance(start: usize, end: usize) -> usize {
    (start as isize - end as isize).abs() as usize
}

pub fn manathan(dist_x: usize, dist_y: usize) -> usize {
    dist_x + dist_y
}

// pub fn euclidienne(dist_x: usize, dist_y: usize) -> usize {
//     ((dist_x.pow(2) + dist_y.pow(2)) as isize).sqrt()
// }

pub fn chebyshev(dist_x: usize, dist_y: usize) -> usize {
    (dist_x + dist_y) - min(dist_x, dist_y)
}

// pub fn octile(dist_x: usize, dist_y: usize) -> usize {
//     (dist_x + dist_y) + ((2 as f64).sqrt() - 2) * min(dist_x, dist_y)
// }
