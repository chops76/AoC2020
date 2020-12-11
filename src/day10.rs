use std::io::{BufRead, BufReader};
use std::fs::File;

type Input = Vec<i64>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| s.parse()).flatten().collect()
}

fn part1(sorted: &Input) -> i64{
	sorted.iter().zip(sorted.iter().skip(1)).filter(|(a, b)| *b-*a==1).count() as i64 *
		sorted.iter().zip(sorted.iter().skip(1)).filter(|(a, b)| *b-*a==3).count() as i64
}

fn part2(ad: &Input) -> i64 {
	let mut dp: Vec<i64> = vec!(0; ad.len());
	dp[0] = 1;

	for i in 1..ad.len() {
		for j in i32::max(0, i as i32 - 3) as usize..i {
			if ad[i] - ad[j] <= 3 {
				dp[i] += dp[j];
			}
		}
	}
	
	dp[ad.len()-1]
}

pub fn main() {
	let mut input = parse_input("./input/day10/input.txt");
	input.sort();
	input.insert(0, 0);
	input.push(input[input.len()-1] + 3);
	
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
		input.insert(0, 0);
		input.push(input[input.len()-1] + 3);
		assert_eq!(part1(&input),35);
	}

	#[test]
	fn day10_test2() {
		let mut input = parse_input("./input/day10/test2.txt");
		input.sort();
		input.insert(0, 0);
		input.push(input[input.len()-1] + 3);
		assert_eq!(part1(&input),220);
	}

	#[test]
	fn day10_test3() {
		let mut input = parse_input("./input/day10/test1.txt");
		input.sort();
		input.insert(0, 0);
		input.push(input[input.len()-1] + 3);
		assert_eq!(part2(&input),8);
	}

	#[test]
	fn day10_test4() {
		let mut input = parse_input("./input/day10/test2.txt");
		input.sort();
		input.insert(0, 0);
		input.push(input[input.len()-1] + 3);
		assert_eq!(part2(&input),19208);
	}
}
