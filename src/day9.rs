use std::io::{BufRead, BufReader};
use std::fs::File;

type Input = Vec<i64>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| s.parse()).flatten().collect()
}

fn part1(nums: &Input, preamble: usize) -> i64 {
	for idx in preamble..nums.len() {
		let mut found = false;
		for check in idx-preamble..idx {
			if nums[check] * 2 != nums[idx] && 
			  nums[idx-preamble..idx].contains(&(nums[idx]-nums[check])) {
				found = true;
				break;
			}
		}
		if !found {
			return nums[idx];
		}
	}
	0
}

fn part2(nums: &Input, target: i64) -> i64 {
	for start in 0..nums.len() {
		let mut idx = start;
		let mut total = 0;
		while total < target && idx < nums.len() {
			total += nums[idx];
			if total == target {
				return nums[start..idx+1].iter().min().unwrap() +
				       nums[start..idx+1].iter().max().unwrap();
			}
			idx += 1;
		}
	}
	0
}

pub fn main() {
	let input = parse_input("./input/day9/input.txt");
	let target = part1(&input, 25);
	println!("Part1: {}", target);
	println!("Part2: {}", part2(&input, target));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day9_test1() {
		let input = parse_input("./input/day9/test.txt");
		assert_eq!(part1(&input, 5),127);
	}

	#[test]
	fn day9_test2() {
		let input = parse_input("./input/day9/test.txt");
		assert_eq!(part2(&input, 127),62);
	}
}
