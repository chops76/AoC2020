use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
	eyr: Option<String>,
	hgt: Option<String>,
    hcl: Option<String>,
	ecl: Option<String>,
	pid: Option<String>,
    cid: Option<String>
}

type Input = Vec<Passport>;

fn parse_input(path: &str) -> Input {
	let f = File::open(path).unwrap();
	let mut items = Vec::new();
	let mut passports = Vec::new();
	
	for line in BufReader::new(f).lines().flatten() {
		if line.len() == 0 {
			passports.push(parse_items(&items));
			items = Vec::new();
		} else {
			for item in line.split(' ') {
				items.push(item.to_string());
			}
		}
	}
	passports.push(parse_items(&items));

	passports
}

fn parse_items(items: &Vec<String>) -> Passport {
	let mut passport: Passport = Default::default();
	for field in items {
		let vals: Vec<&str> = field.split(':').collect();
		match vals[0] {
			"byr" => passport.byr = Some(vals[1].to_string()),
			"iyr" => passport.iyr = Some(vals[1].to_string()),
			"eyr" => passport.eyr = Some(vals[1].to_string()),
			"hgt" => passport.hgt = Some(vals[1].to_string()),
			"hcl" => passport.hcl = Some(vals[1].to_string()),
			"ecl" => passport.ecl = Some(vals[1].to_string()),
			"pid" => passport.pid = Some(vals[1].to_string()),
			"cid" => passport.cid = Some(vals[1].to_string()),
			_ => println!("Something terrible happened: {}", vals[0])	
		}
	}
	passport
}

fn valid_pp(passport: &Passport) -> bool {
	passport.byr != None && passport.iyr != None && passport.eyr != None &&
		passport.hgt != None && passport.hcl != None && passport.ecl != None &&
		passport.pid != None 
}

fn valid_pp2(passport: &Passport) -> bool {
	if !(passport.byr != None && passport.iyr != None && passport.eyr != None &&
		passport.hgt != None && passport.hcl != None && passport.ecl != None &&
		passport.pid != None) {
			return false;
	}

	let byr = passport.byr.as_ref().unwrap().parse();
	if byr.is_err() {
		return false;
	}
	let byr:u32 = byr.unwrap();
	if byr < 1920 || byr > 2002 {
		return false;
	}

	let iyr = passport.iyr.as_ref().unwrap().parse();
	if iyr.is_err() {
		return false;
	}
	let iyr:u32 = iyr.unwrap();
	if iyr < 2010 || iyr > 2020 {
		return false;
	}

	let eyr = passport.eyr.as_ref().unwrap().parse();
	if eyr.is_err() {
		return false;
	}
	let eyr:u32 = eyr.unwrap();
	if eyr < 2020 || eyr > 2030 {
		return false;
	}

	let hgt = passport.hgt.as_ref().unwrap();
	if &hgt[hgt.len()-2..] == "cm" {
		let val:u32 = hgt[..hgt.len()-2].parse().unwrap();
		if val < 150 || val > 193 {
			return false;
		}
	} else 	if &hgt[hgt.len()-2..] == "in" {
		let val:u32 = hgt[..hgt.len()-2].parse().unwrap();
		if val < 59 || val > 76 {
			return false;
		}
	} else {
		return false;
	}

	let hcl = passport.hcl.as_ref().unwrap().as_bytes();
	if hcl.len() != 7 {
		return false;
	}
	if hcl[0] != b'#' {
		return false;
	}
	for i in 1..7 {
		if !(hcl[i] as char).is_alphanumeric() {
			return false;
		}
	}

	let ecl = passport.ecl.as_ref().unwrap();
	if ecl != "amb" && ecl != "blu" && ecl != "brn" && ecl != "gry" &&
	   ecl != "grn" && ecl != "hzl" && ecl != "oth" {
		   return false;
	   }
	
	let pid = passport.pid.as_ref().unwrap().as_bytes();
	if pid.len() != 9 {
		return false;
	}
	for i in 0..9 {
		if !(pid[i] as char).is_numeric() {
			return false;
		}
	}
	true
}

pub fn main() {
	let input = parse_input("./input/day4/input.txt");
	let num_valid = input.iter().filter(|p| valid_pp(p)).count();
	println!("Part 1: {}", num_valid);
	let num_valid2 = input.iter().filter(|p| valid_pp2(p)).count();
	println!("Part 2: {}", num_valid2);
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn day4_test1() {
		assert_eq!(0,0); 
	}
}
