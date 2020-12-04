use lazy_static::lazy_static;
use std::collections::HashSet;
use std::io::Read;
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
	let mut fstr = String::new();
	File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
	let mut passports = Vec::new();

	for pp in fstr.split("\n\n") {
		passports.push(parse_items(pp.split_ascii_whitespace().collect()));	
	}
	passports
}

fn parse_items(items: Vec<&str>) -> Passport {
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

fn validate_byr(byr: &str) -> bool {
	byr.len() == 4 && (1920..=2002).contains(&byr.parse::<u32>().unwrap())
}

fn validate_iyr(iyr: &str) -> bool {
	iyr.len() == 4 && (2010..=2020).contains(&iyr.parse::<u32>().unwrap())
}

fn validate_eyr(eyr: &str) -> bool {
	eyr.len() == 4 && (2020..=2030).contains(&eyr.parse::<u32>().unwrap())
}

fn validate_hgt(hgt: &str) -> bool {
	match hgt.split_at(hgt.len() - 2) {
		(h, "cm") => (150..=193).contains(&h.parse::<u32>().unwrap()),
		(h, "in") => (59..=76).contains(&h.parse::<u32>().unwrap()),
		_ => false
	}
}

fn validate_hcl(hcl: &str) -> bool {
	hcl.chars().next() == Some('#') &&
		hcl.chars().skip(1).all(|c| !c.is_uppercase() && c.is_ascii_hexdigit() )
}

fn validate_ecl(ecl: &str) -> bool {
	lazy_static! {
		static ref VALID_ECR: HashSet<&'static str> =
			["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().collect();
	}

	VALID_ECR.contains(ecl)
}

fn validate_pid(pid: &str) -> bool {
	pid.len() == 9 && pid.chars().all(|c| c.is_numeric())
}

fn valid_pp2(passport: &Passport) -> bool {
	valid_pp(passport) && 
		validate_byr(&passport.byr.as_ref().unwrap()) &&
		validate_iyr(&passport.iyr.as_ref().unwrap()) && 
		validate_eyr(&passport.eyr.as_ref().unwrap()) &&
		validate_hgt(&passport.hgt.as_ref().unwrap()) &&
		validate_hcl(&passport.hcl.as_ref().unwrap()) &&
		validate_ecl(&passport.ecl.as_ref().unwrap()) &&
		validate_pid(&passport.pid.as_ref().unwrap())
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
