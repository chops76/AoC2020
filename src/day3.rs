use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Grid = Vec<Vec<bool>>;

fn parse_line(s: &str) -> Vec<bool> {
    s.bytes().map(|b| b == b'#').collect()
}

fn parse_input(path: &str) -> Grid {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn count_trees(grid: &Grid, x_step: usize, y_step: usize) -> u32 {
	(0..grid.len()).step_by(y_step).enumerate().
	    filter(|(count, y)| grid[*y][(count*x_step)%grid[0].len()]).count() as u32
}

pub fn main() {
	let grid = parse_input("./input/day3/input.txt");

	let p1_timer = Instant::now();
    let p1_result = count_trees(&grid, 3, 1);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = count_trees(&grid, 1, 1) * count_trees(&grid, 3, 1) *
		count_trees(&grid, 5, 1) * count_trees(&grid, 7, 1) *
		count_trees(&grid, 1, 2);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day3_test1() {
		let test_grid = parse_input("./input/day3/test.txt");
		assert_eq!(count_trees(&test_grid, 1, 1),2); 
	}

	#[test]
	fn day3_test2() {
		let test_grid = parse_input("./input/day3/test.txt");
		assert_eq!(count_trees(&test_grid, 3, 1),7); 
	}

	#[test]
	fn day3_test3() {
		let test_grid = parse_input("./input/day3/test.txt");
		assert_eq!(count_trees(&test_grid, 5, 1),3); 
	}

	#[test]
	fn day3_test4() {
		let test_grid = parse_input("./input/day3/test.txt");
		assert_eq!(count_trees(&test_grid, 7, 1),4); 
	}

	#[test]
	fn day3_test5() {
		let test_grid = parse_input("./input/day3/test.txt");
		assert_eq!(count_trees(&test_grid, 1, 2),2); 
	}
}
