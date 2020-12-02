use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::fs::File;

type Input = Vec<u32>;

fn parse_input() -> Input {
    let path = "./input/day1/input1.txt";
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| s.parse().unwrap()).collect()
}

fn part1(nums: &Input) -> u32 {
	nums.iter().combinations(2).find(|x| x.iter().copied().sum::<u32>() == 2020).unwrap()
	.iter().copied().product()
}

fn part2(nums: &Input) -> u32 {
	nums.iter().combinations(3).find(|x| x.iter().copied().sum::<u32>() == 2020).unwrap()
	.iter().copied().product()
}

pub fn main() {
	let nums = parse_input();

	println!("Part 1: {}", part1(&nums));
	println!("Part 2: {}", part2(&nums));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day1_test1() {
		assert_eq!(part1(&vec![1721,979,366,299,675,1456]), 514579);
	}

	#[test]
	fn day1_test2() {
		assert_eq!(part2(&vec![1721,979,366,299,675,1456]), 241861950);
	}
}
