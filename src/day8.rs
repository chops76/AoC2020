use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

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

fn part2(prog: &Input) -> i32 {
	for ip in 0..prog.len() {
		let mut modified = prog.to_owned();
		if modified[ip].ins == "nop" {
			modified[ip].ins = "jmp".to_string();
		} else if modified[ip].ins == "jmp" {
			modified[ip].ins = "nop".to_string();
		}
		let (acc, final_ip) = run_prog(&modified);
		if final_ip == modified.len() as i32 {
			return acc;
		}
	}
	0
}

pub fn main() {
	let input = parse_input("./input/day8/input.txt");

	let p1_timer = Instant::now();
	let (p1_result, _) = run_prog(&input);
    let p1_time = p1_timer.elapsed();
	println!("Part 1: {}", p1_result);
	println!("Part 1 Time: {:?}", p1_time);

	let p2_timer = Instant::now();
    let p2_result = part2(&input);
    let p2_time = p2_timer.elapsed();
	println!("Part 2: {}", p2_result);
	println!("Part 2 Time: {:?}", p2_time);
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
