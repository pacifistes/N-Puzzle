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

// pub fn linear_conflict(actual_state: &Vec<u8>, goal_state: &Vec<u8>, size: u8) -> u16 {
	// for value in 0..actual_state.len() {
	// 	// self.state.iter().position(|&r| r == value).unwrap() as u16
	// 	let actual_index = actual_state.iter().position(|&r| r == value).unwrap();
	// 	let goal_index = goal_state.get_index_of_value(value);
	// 	let actual_coordinates = (actual_index / size, actual_index % size)
	// 	let goal_coordinates = (goal_index / size, goal_index % size)
	// 	if (actual_coordinates.0 == goal_coordinates.0) {
	// 	}
	// 	if (actual_coordinates.1 == goal_coordinates.1) {

	// 	}
	// }
	// for y in 0..size {
	// 	for x in 0..size {
	// 		if value == 0 {
    //     	    continue;
    //     	}
	//         let actual_index = self.get_index_of_value(value) as usize;
	//         let goal_index = goal.get_index_of_value(value) as usize;
	//         let dist_x = distance(self.get_x(actual_index), goal.get_x(goal_index));
	//         let dist_y = distance(self.get_y(actual_index), goal.get_y(goal_index));
	// 	}
	// }
	// 0
// }