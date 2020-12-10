use std::io::{BufRead, BufReader};
use std::fs::File;

type Input = Vec<i64>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| s.parse()).flatten().collect()
}

fn recur(cur_idx: usize, target: i64, adapters: &Input, memo: &mut Vec<i64>) -> i64 {
	if memo[cur_idx] != -1 {
		return memo[cur_idx];
	}
	let mut sum = 0;
	if target - adapters[cur_idx] > 0 && target - adapters[cur_idx] <= 3 {
		sum += 1;
	}
	let mut chk_idx = cur_idx + 1;
	while chk_idx < adapters.len() && adapters[chk_idx] - adapters[cur_idx] <= 3 {
		sum += recur(chk_idx, target, adapters, memo);
		chk_idx += 1;
	}
	memo[cur_idx] = sum;
	sum
}

fn part1(adapters: &Input) -> i64{
	let mut sorted = adapters.to_owned();
	sorted.push(sorted[sorted.len()-1] + 3);
	let mut ones = 0;
	let mut threes = 0;
	let mut cur_val = 0;
	for val in sorted {
		if val - cur_val == 1 {
			ones += 1;
		} else if val - cur_val == 3 {
			threes += 1;
		}
		cur_val = val;
	}
	ones * threes
}

fn part2(adapters: &Input) -> i64 {
	let mut memo: Vec<i64> = vec!(-1; adapters.len());
	let mut sum = 0;
	let mut idx = 0;

	let target = *adapters.iter().max().unwrap() + 3;
	while adapters[idx] <= 3 {
		sum += recur(idx, target, &adapters, &mut memo);
		idx += 1;
	}
	sum
}

pub fn main() {
	let mut input = parse_input("./input/day10/input.txt");
	input.sort();
	println!("Part 1: {}", part1(&input));
	println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day10_test1() {
		let mut input = parse_input("./input/day10/test1.txt");
		input.sort();
		assert_eq!(part1(&input),35);
	}

	#[test]
	fn day10_test2() {
		let mut input = parse_input("./input/day10/test2.txt");
		input.sort();
		assert_eq!(part1(&input),220);
	}

	#[test]
	fn day10_test3() {
		let mut input = parse_input("./input/day10/test1.txt");
		input.sort();
		assert_eq!(part2(&input),8);
	}

	#[test]
	fn day10_test4() {
		let mut input = parse_input("./input/day10/test2.txt");
		input.sort();
		assert_eq!(part2(&input),19208);
	}
}
