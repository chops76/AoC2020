use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
#[derive(Clone)]
struct Entry {
	ins: String,
	val: i32
}

type Input = Vec<Entry>;

fn parse_line(s: &str) -> Entry {
    let spl: Vec<&str> = s.split(" ").collect();
	Entry {
		ins: spl[0].to_string(),
		val: spl[1].parse().unwrap()
	}
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn run_prog(prog: &Input) -> (i32, i32) {
	let mut ip: i32 = 0;
	let mut acc: i32 = 0;
	let mut executed = vec!(false; prog.len());
	while ip < prog.len() as i32 && !executed[ip as usize] {
		executed[ip as usize] = true;
		match &prog[ip as usize].ins[..] {
			"nop" => { ip += 1; }
			"jmp" => { ip += prog[ip as usize].val }
			"acc" => { acc += prog[ip as usize].val; ip += 1;}
			_ => { println!("<< UNKNOWN INSTRUCTION >>") }
		}
	}
	(acc, ip)
}

pub fn main() {
	let input = parse_input("./input/day8/input.txt");
	let (acc, _) = run_prog(&input);
	println!("Part 1: {}", acc);

	for ip in 0..input.len() {
		let mut modified = input.to_owned();
		if modified[ip].ins == "nop" {
			modified[ip].ins = "jmp".to_string();
		} else if modified[ip].ins == "jmp" {
			modified[ip].ins = "nop".to_string();
		}
		let (acc, final_ip) = run_prog(&modified);
		if final_ip == modified.len() as i32 {
			println!("Part 2: {}", acc);
			break;
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day8_test1() {
		let input = parse_input("./input/day8/test.txt");
		let (acc, _) = run_prog(&input);
		assert_eq!(acc,5);
	}

	#[test]
	fn day8_test2() {
		let input = parse_input("./input/day8/test.txt");
		let (_, ip) = run_prog(&input);
		assert_eq!(ip,1);
	}
}
