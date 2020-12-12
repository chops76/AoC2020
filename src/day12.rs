use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
struct Command {
	direction: char,
	distance: i32
}

type Input = Vec<Command>;

fn parse_line(s: &str) -> Command {
	Command {
		direction: s.chars().next().unwrap(),
		distance: s[1..].parse().unwrap()
	}
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn part1(cmds: &Input) -> i32 {
	let mut dir = 0;
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	for cmd in cmds {
		match cmd.direction {
			'N' => y += cmd.distance,
			'E' => x += cmd.distance,
			'S' => y -= cmd.distance,
			'W' => x -= cmd.distance,
			'R' => dir = (dir + cmd.distance / 90) % 4,
			'L' => dir = (dir + 4 - cmd.distance / 90) % 4,
			'F' => match dir {
				0 => x += cmd.distance,
				1 => y -= cmd.distance,
				2 => x -= cmd.distance,
				3 => y += cmd.distance,
				_ => println!("<<INVALID DIR>>")
			}
			_ => println!("<<INVALID COMMAND>>")
		}
	}
	x.abs() + y.abs()
}

fn part2(cmds: &Input) -> i32 {
	let mut way_x = 10;
	let mut way_y = 1;
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	for cmd in cmds {
		match cmd.direction {
			'N' => way_y += cmd.distance,
			'E' => way_x += cmd.distance,
			'S' => way_y -= cmd.distance,
			'W' => way_x -= cmd.distance,
			'R' => {
				for _ in 0..cmd.distance / 90 {
					way_x ^= way_y;
					way_y ^= way_x;
					way_x ^= way_y;
					way_y *= -1;
				}
			}
			'L' => {
				for _ in 0..cmd.distance / 90 {
					way_x ^= way_y;
					way_y ^= way_x;
					way_x ^= way_y;
					way_x *= -1;
				}
			}
			'F' => {
				x += way_x * cmd.distance; 
				y += way_y * cmd.distance;
			}
			_ => println!("<<INVALID COMMAND>>")
		}
	}
	x.abs() + y.abs()
}

pub fn main() {
	let input = parse_input("./input/day12/input.txt");
	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day12_test1() {
		let input = parse_input("./input/day12/test.txt");
		assert_eq!(part1(&input),25);
	}

	#[test]
	fn day12_test2() {
		let input = parse_input("./input/day12/test.txt");
		assert_eq!(part2(&input),286);
	}
}
