use std::cmp::min;

#[warn(dead_code)]
pub enum Heuristic {
    MANATHAN,
    CHEBYSHEV,
    EUCLIDIENNE,
    OCTILE,
    HAMMING,
    LINEAR_CONFLICT,
}

pub fn distance(start: usize, end: usize) -> u16 {
    (start as i16 - end as i16).abs() as u16
}

pub fn manathan(dist_x: u16, dist_y: u16) -> u16 {
    dist_x + dist_y
}

pub fn euclidienne(dist_x: u16, dist_y: u16) -> u16 {
    f32::from(dist_x.pow(2) + dist_y.pow(2)).sqrt() as u16
}

pub fn chebyshev(dist_x: u16, dist_y: u16) -> u16 {
    (f32::from(dist_x + dist_y) - f32::from(min(dist_x, dist_y))) as u16
}

pub fn octile(dist_x: u16, dist_y: u16) -> u16 {
    (f32::from(dist_x + dist_y) + ((2 as f32).sqrt() - 2.0) * f32::from(min(dist_x, dist_y))) as u16
}

pub fn hamming(dist_x: u16, dist_y: u16) -> u16 {
    match dist_x + dist_y {
        0 => 0,
        _ => 1,
    }
}

pub fn linear_conflict(actual_index: &Vec<u8>, goal_index: &Vec<u8>, size: u8) -> u16 {
    let mut weight = 0;
	let range: Vec<usize> = (1..size as usize).collect();
	let all_coordinate: Vec<((u8,u8), (u8,u8))> = range.iter().map(|i| ((actual_index[*i] / size, actual_index[*i] % size), (goal_index[*i] / size, goal_index[*i] % size))).collect();
	all_coordinate.iter().for_each(|(actual_coordinate, goal_coordinate)| {
			if actual_coordinate.0 == goal_coordinate.0 {
					all_coordinate.iter().filter(|(other_coordinate, other_goal_coordinate)|
					other_coordinate.0 == actual_coordinate.0
					&& other_coordinate.0 == other_goal_coordinate.0
					&& other_coordinate.1 > actual_coordinate.1
					&& other_goal_coordinate.1 < goal_coordinate.1
					).for_each(|(_,_)| weight += 2);

			}
			if actual_coordinate.1 == goal_coordinate.1 {
					all_coordinate.iter().filter(|(other_coordinate, other_goal_coordinate)|
					other_coordinate.1 == actual_coordinate.1
					&& other_coordinate.1 == other_goal_coordinate.1
					&& other_coordinate.0 > actual_coordinate.0
					&& other_goal_coordinate.0 < goal_coordinate.0
					).for_each(|(_,_)| weight += 2);
			}
		}
	);
    weight
}
