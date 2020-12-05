use std::io::{BufRead, BufReader};
use std::fs::File;

type Input = Vec<String>;

fn parse_input() -> Input {
    let path = "./input/day5/input.txt";
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn get_seat(code: &str) -> u32
{
	let fb: u32 = code[..7].chars().rev().enumerate()
		.map(|(p, v)| if v == 'B' {u32::pow(2,p as u32)} else {0}).sum();
	let lr: u32 = code[7..].chars().rev().enumerate()
		.map(|(p, v)| if v == 'R' {u32::pow(2,p as u32)} else {0}).sum();
	fb * 8 + lr
}

pub fn main() {
	let seats = parse_input();
	let mut ids = seats.iter().map(|s| get_seat(s)).collect::<Vec<u32>>();
	ids.sort();
	println!("Part 1: {}", ids.iter().max().unwrap());
	println!("Part 2: {}", ids.iter().zip(ids.iter().skip(1))
		.filter(|(a, b)| **b == *a + 2 ).map(|(a, _)| a ).sum::<u32>() + 1);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day5_test1() {
		assert_eq!(get_seat("FBFBBFFRLR"), 357);
	}

	#[test]
	fn day5_test2() {
		assert_eq!(get_seat("BFFFBBFRRR"), 567);
	}

	#[test]
	fn day5_test3() {
		assert_eq!(get_seat("FFFBBBFRRR"), 119);
	}

	#[test]
	fn day5_test4() {
		assert_eq!(get_seat("BBFFBBFRLL"), 820);
	}
}
