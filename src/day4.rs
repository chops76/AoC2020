use std::io::Read;
use std::fs::File;
use crate::passport::Passport;

type Input = Vec<Passport>;

fn parse_input(path: &str) -> Input {
	let mut fstr = String::new();
	File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
	let mut passports = Vec::new();

	for pp in fstr.split("\n\n") {
		passports.push(Passport::new(pp.split_ascii_whitespace().collect()));	
	}
	passports
}

pub fn main() {
	let input = parse_input("./input/day4/input.txt");
	let num_valid = input.iter().filter(|p| p.fields_populated()).count();
	println!("Part 1: {}", num_valid);
	let num_valid2 = input.iter().filter(|p| p.validate()).count();
	println!("Part 2: {}", num_valid2);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day4_test1() {
		let input = parse_input("./input/day4/test.txt");
		assert_eq!(input.iter().filter(|p| p.fields_populated()).count(),2); 
	}

	#[test]
	fn day4_test2() {
		let input = parse_input("./input/day4/test.txt");
		assert_eq!(input.iter().filter(|p| p.validate()).count(),2); 
	}
}
