use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

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
	let people = answers.split("\n");
	let sets = people.map(|s| s.chars().collect()).collect::<Vec<HashSet<char>>>();
	let mut work_set = answers.chars().filter(|c| c.is_alphabetic()).collect::<HashSet<char>>();
	for i in 0..sets.len() {
		work_set = work_set.intersection(&sets[i]).copied().collect::<HashSet<char>>();
	}
	work_set.len()
}

pub fn main() {
	let input = parse_input("./input/day6/input.txt");
	println!("Part 1: {}", input.iter().map(|s| num_yes(&s)).sum::<usize>());
	println!("Part 2: {}", input.iter().map(|s| num_all(&s)).sum::<usize>());
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
