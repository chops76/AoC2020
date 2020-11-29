use std::fs::read_to_string;

fn test_add_1(val: u64) -> u64 {
	val + 1
}

pub fn main() {
	let input = read_to_string("input/day1/input1.txt").unwrap();

	let mut input_sum = 0;

	for line in input.lines() {
		let num: u64 = line.parse().unwrap();
		input_sum += test_add_1(num);
	}

	println!("Boilerplate Input: {}", input_sum);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day1_test1() {
		assert_eq!(1, test_add_1(0));
	}

	#[test]
	fn day1_test2() {
		assert_eq!(1000, test_add_1(999));
	}

	#[test]
	fn day1_test3() {
		assert_eq!(101, test_add_1(100));
	}
}
