use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
#[derive(PartialEq)]
struct Password {
    pw_text: String,
    letter: u8,
    let_max: u32,
    let_min: u32,
}

type Input = Vec<Password>;

fn parse_input() -> Input {
    let path = "./input/day2/input.txt";
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(s: &str) -> Password {
	let spl: Vec<&str> = s.split(' ').collect();
	let nums: Vec<&str> = spl[0].split('-').collect();
    Password {
        pw_text: spl[2].to_string(),
        let_min: nums[0].parse().unwrap(),
        let_max: nums[1].parse().unwrap(),
        letter: spl[1].as_bytes()[0],
    }
}

fn num_valid1(pw_rules: &Input) -> u32 {
	pw_rules.iter().filter(|r| valid1(r)).count() as u32
}

fn valid1(rule: &Password) -> bool
{
	let mut ret_val = false;
	let num_found = rule.pw_text.as_bytes().iter()
		                .filter(|&n| *n==rule.letter).count() as u32;
	if num_found >= rule.let_min && num_found <= rule.let_max {
		ret_val = true;
	}

	ret_val
}

fn num_valid2(pw_rules: &Input) -> u32 {
	pw_rules.iter().filter(|r| valid2(r)).count() as u32
}

fn valid2(rule: &Password) -> bool
{
	let mut result_sum = 0;
	if rule.pw_text.as_bytes()[(rule.let_min-1) as usize] == rule.letter {
		result_sum += 1;
	}
	if rule.pw_text.as_bytes()[(rule.let_max-1) as usize] == rule.letter {
		result_sum += 1;
	}

	result_sum == 1
}

pub fn main() {
	let pw_rules = parse_input();
	println!("Part 1: {}", num_valid1(&pw_rules));
	println!("Part 2: {}", num_valid2(&pw_rules));
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day2_test1() {
		assert_eq!(parse_line("14-15 d: dzjgbdwdkdhdddh"), 
        Password {
            pw_text: "dzjgbdwdkdhdddh".to_string(),
            let_min: 14,
            let_max: 15,
            letter: b'd'            
        });
	}

	#[test]
	fn day2_test2() {
		let tmp = Password {
            pw_text: "nnnnnnnnn".to_string(),
            let_min: 8,
            let_max: 9,
            letter: b'n'            
        };
		assert_eq!(valid1(&tmp),true);
	}

	#[test]
	fn day2_test3() {
		let tmp = Password {
            pw_text: "dzjgbdwdkdhdddh".to_string(),
            let_min: 14,
            let_max: 15,
            letter: b'd'            
        };
		assert_eq!(valid1(&tmp),false);
	}

	#[test]
	fn day2_test4() {
		let tmp = Password {
            pw_text: "dzjgbdwdkdhdddh".to_string(),
            let_min: 14,
            let_max: 15,
            letter: b'd'            
        };
		assert_eq!(valid2(&tmp),true);
	}

	#[test]
	fn day2_test5() {
		let tmp = Password {
            pw_text: "nnnnnnnnn".to_string(),
            let_min: 8,
            let_max: 9,
            letter: b'n'            
        };
		assert_eq!(valid2(&tmp),false);
	}
}
