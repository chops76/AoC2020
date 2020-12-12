use std::io::{BufRead, BufReader};
use std::fs::File;
use num::complex::Complex;

#[derive(Debug)]
struct Command {
	direction: char,
	distance: i32
}

type Input = Vec<Command>;

static NORTH:Complex<i32> = Complex::new(0, 1);
static SOUTH:Complex<i32> = Complex::new(0, -1);
static EAST:Complex<i32> = Complex::new(1, 0);
static WEST:Complex<i32> = Complex::new(-1, 0);
static NEG_I:Complex<i32> = Complex::new(0, -1);

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
	let mut dir = Complex::new(1, 0);
	let mut pos = Complex::new(0, 0);
	for cmd in cmds {
		match cmd.direction {
			'N' => pos += NORTH * cmd.distance,
			'E' => pos += EAST * cmd.distance,
			'S' => pos += SOUTH * cmd.distance,
			'W' => pos += WEST * cmd.distance,
			'R' => dir *= NEG_I.powi(cmd.distance / 90),
			'L' => dir *= NEG_I.powi(-cmd.distance / 90),
			'F' => pos += dir * cmd.distance,
			_ => println!("<<INVALID COMMAND>>")
			}
		}	
	pos.re.abs() + pos.im.abs()
}

fn part2(cmds: &Input) -> i32 {
	let mut dir = Complex::new(10, 1);
	let mut pos = Complex::new(0, 0);
	for cmd in cmds {
		match cmd.direction {
			'N' => dir += NORTH * cmd.distance,
			'E' => dir += EAST * cmd.distance,
			'S' => dir += SOUTH * cmd.distance,
			'W' => dir += WEST * cmd.distance,
			'R' => dir *= NEG_I.powi(cmd.distance / 90),
			'L' => dir *= NEG_I.powi(-cmd.distance / 90),
			'F' => pos += dir * cmd.distance,
			_ => println!("<<INVALID COMMAND>>")
			}
		}	
	pos.re.abs() + pos.im.abs()
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
