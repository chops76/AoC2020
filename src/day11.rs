use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Chair {
	Empty,
	Occupied,
	Floor
}

type Grid = Vec<Vec<Chair>>;

fn parse_line(s: &str) -> Vec<Chair> {
	s.bytes().map(|b| match b { b'L' => Chair::Empty, 
								b'#' => Chair::Occupied, 
								_ => Chair::Floor }).collect()
}

fn parse_input(path: &str) -> Grid {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn neighbors_occupied(grid: &Grid, x: usize, y: usize) -> u32 {
	let mut sum = 0;
	if x > 0 && grid[y][x-1] == Chair::Occupied {
		sum += 1;
	}
	if x > 0 && y > 0 && grid[y-1][x-1] == Chair::Occupied {
		sum += 1;
	}
	if y > 0 && grid[y-1][x] == Chair::Occupied {
		sum += 1;
	}
	if y > 0 && x < grid[0].len() - 1 && grid[y-1][x+1] == Chair::Occupied {
		sum += 1;
	}
	if x < grid[0].len() - 1 && grid[y][x+1] == Chair::Occupied {
		sum += 1;
	}
	if x < grid[0].len() - 1 && y < grid.len() - 1 && grid[y+1][x+1] == Chair::Occupied {
		sum += 1;
	}
	if y < grid.len() - 1 && grid[y+1][x] == Chair::Occupied {
		sum += 1;
	}
	if x > 0 && y < grid.len() - 1 && grid[y+1][x-1] == Chair::Occupied {
		sum += 1;
	}
	sum
}

fn scan(grid: &Grid, start_x: usize, start_y: usize, x_incr: i32, y_incr: i32) -> bool
{
	let mut x = start_x as i32 + x_incr;
	let mut y = start_y as i32 + y_incr;
	while x >= 0 && y >= 0 && x < grid[0].len() as i32 && y < grid.len() as i32 {
		if grid[y as usize][x as usize] == Chair::Occupied {
			return true;
		} else if grid[y as usize][x as usize] == Chair::Empty {
			return false;
		}
		x += x_incr;
		y += y_incr;
	}
	false
}

fn neighbors_occupied2(grid: &Grid, x: usize, y: usize) -> u32 {
	let mut sum = 0;
	if scan(grid, x, y, -1, 0) {
		sum += 1;
	}
	if scan(grid, x, y, -1, -1) {
		sum += 1;
	}
	if scan(grid, x, y, 0, -1) {
		sum += 1;
	}
	if scan(grid, x, y, 1, -1) {
		sum += 1;
	}
	if scan(grid, x, y, 1, 0) {
		sum += 1;
	}
	if scan(grid, x, y, 1, 1) {
		sum += 1;
	}
	if scan(grid, x, y, 0, 1) {
		sum += 1;
	}
	if scan(grid, x, y, -1, 1) {
		sum += 1;
	}
	sum
}

fn next_step(old_grid: &Grid, part2: bool) -> Grid {
	let mut new_grid = old_grid.clone();

	for y in 0..new_grid.len() {
		for x in 0..new_grid[y].len() {
			let neighbors = match part2 {
				false => neighbors_occupied(&old_grid, x, y),
				true => neighbors_occupied2(&old_grid, x, y) };
			if old_grid[y][x] == Chair::Empty && neighbors == 0 {
				new_grid[y][x] = Chair::Occupied;
			} else if old_grid[y][x] == Chair::Occupied && 
					  neighbors >= match part2 { false=>4,true=>5} {
				new_grid[y][x] = Chair::Empty;
			}
		}
	}

	new_grid
}

fn num_occupied(grid: &Grid) -> u32 {
	let mut sum = 0;
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] == Chair::Occupied {
				sum += 1;
			}
		}
	}
	sum
}

fn grid_loop(input: &Grid, part2: bool) -> u32 {
	let mut cur_input = input.clone();
	let mut new_input;
	loop {
		new_input = next_step(&cur_input, part2);
		if new_input == cur_input {
			break;
		}
		cur_input = new_input;
	}
	num_occupied(&new_input)
}

pub fn main() {
	let input = parse_input("./input/day11/input.txt");

	println!("Part 1: {}", grid_loop(&input, false));
	println!("Part 2: {}", grid_loop(&input, true));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day11_test1() {
		let input = parse_input("./input/day11/test.txt");
		assert_eq!(grid_loop(&input, false),37);
	}

	#[test]
	fn day11_test2() {
		let input = parse_input("./input/day11/test.txt");
		assert_eq!(grid_loop(&input, true),26);
	}
}
