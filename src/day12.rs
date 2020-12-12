use std::io::{BufRead, BufReader};
use std::fs::File;
use num::complex::Complex;
use std::time::Instant;

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

fn calc(cmds: &Input, p2: bool) -> i32 {
	let mut dir = match p2 {
		true => Complex::new(10,1),
		false => Complex::new(1,0)
		};
	let mut pos = Complex::new(0, 0);
	for cmd in cmds {
		let mut delta = Complex::new(0, 0);
		match cmd.direction {
			'N' => delta = NORTH * cmd.distance,
			'E' => delta = EAST * cmd.distance,
			'S' => delta = SOUTH * cmd.distance,
			'W' => delta = WEST * cmd.distance,
			'R' => dir *= NEG_I.powi(cmd.distance / 90),
			'L' => dir *= NEG_I.powi(-cmd.distance / 90),
			'F' => pos += dir * cmd.distance,
			_ => println!("<<INVALID COMMAND>>")
			}
		match p2 {
			true => dir += delta,
			false => pos += delta
			}
		}	
	pos.re.abs() + pos.im.abs()
}

pub fn main() {
	let input = parse_input("./input/day12/input.txt");

	let p1_timer = Instant::now();
    let p1_result = calc(&input, false);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = calc(&input, true);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day12_test1() {
		let input = parse_input("./input/day12/test.txt");
		assert_eq!(calc(&input, false),25);
	}

	#[test]
	fn day12_test2() {
		let input = parse_input("./input/day12/test.txt");
		assert_eq!(calc(&input, true),286);
	}
}
