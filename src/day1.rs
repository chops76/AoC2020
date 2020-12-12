use itertools::Itertools;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

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

	let p1_timer = Instant::now();
    let p1_result = part1(&nums);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&nums);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
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
