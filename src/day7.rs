use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
struct Entry {
	name: String,
	number: usize
}

type Input = HashMap<String, Vec<Entry>>;

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	let mut ret_map: Input = Input::new();
	for line in BufReader::new(f).lines().flatten() {
		let words: Vec<&str> = line.split(" ").collect();
		let first = format!("{} {}", words[0], words[1]);
		let mut sec_vec: Vec<Entry> = Vec::new();
		let mut idx = 4;
		while idx < words.len() && words[idx] != "no" {
			let ent = Entry {
				name: format!("{} {}", words[idx+1], words[idx+2]),
				number: words[idx].parse().unwrap()
			};
			sec_vec.push(ent);
			idx += 4;
		}
		ret_map.insert(first, sec_vec);
	}
	ret_map
}

fn parent_of_shiny_gold(string: &str, input: &Input) -> bool {
	if string == "shiny gold" {
		return true;
	}
	for item in input[string].iter() {
		if parent_of_shiny_gold(&item.name, &input) {
			return true;
		}
	}
	false
}

fn num_inside(string: &str, input: &Input) -> usize {
	let mut sum = 0;
	for item in input[string].iter() {
		sum += item.number * (1 + num_inside(&item.name, &input));
	}
	sum
}

pub fn main() {
	let input = parse_input("./input/day7/input.txt");
	println!("Part 1: {}", input.iter().filter(|(name, _)| parent_of_shiny_gold(name, &input)).count() - 1);
	println!("Part 2: {}", num_inside("shiny gold", &input));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day7_test1() {
		let input = parse_input("./input/day7/test.txt");
		assert_eq!(input.iter().filter(|(name, _)| parent_of_shiny_gold(name, &input)).count() - 1, 4);
	}

	#[test]
	fn day7_test2() {
		let input = parse_input("./input/day7/test.txt");
		assert_eq!(num_inside("shiny gold", &input), 32);
	}
}
