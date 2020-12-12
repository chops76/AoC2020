use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::time::Instant;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
	let mut fstr = String::new();
	File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

	fstr.split("\n\n").map(|s| s.to_string()).collect() 
}

fn num_yes(answers: &str) -> usize {
	answers.chars().filter(|c| c.is_alphabetic()).collect::<HashSet<char>>().len()
}

fn num_all(answers: &str) -> usize {
	let sets = answers.split("\n").map(|s| s.chars().collect()).collect::<Vec<HashSet<char>>>();
	sets.iter().fold(sets[0].to_owned(), |i,j| i.intersection(j).copied().collect::<HashSet<char>>()).len()
}

pub fn main() {
	let input = parse_input("./input/day6/input.txt");

	let p1_timer = Instant::now();
    let p1_result = input.iter().map(|s| num_yes(&s)).sum::<usize>();
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = input.iter().map(|s| num_all(&s)).sum::<usize>();
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day6_test1() {
		let input = parse_input("./input/day6/test.txt");
		assert_eq!(input.iter().map(|s| num_yes(&s)).sum::<usize>(),11);
	}

	#[test]
	fn day6_test2() {
		let input = parse_input("./input/day6/test.txt");
		assert_eq!(input.iter().map(|s| num_all(&s)).sum::<usize>(),6);
	}
}
