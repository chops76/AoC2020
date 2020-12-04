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

	#[test]
	fn day4_test3() {
		let pp = Passport::new(vec!["byr:2002"]);
		assert_eq!(pp.validate_byr(), true); 
	}

	#[test]
	fn day4_test4() {
		let pp = Passport::new(vec!["byr:2003"]);
		assert_eq!(pp.validate_byr(), false); 
	}

	#[test]
	fn day4_test5() {
		let pp = Passport::new(vec!["hgt:60in"]);
		assert_eq!(pp.validate_hgt(), true); 
	}

	#[test]
	fn day4_test6() {
		let pp = Passport::new(vec!["hgt:190cm"]);
		assert_eq!(pp.validate_hgt(), true); 
	}

	#[test]
	fn day4_test7() {
		let pp = Passport::new(vec!["hgt:190in"]);
		assert_eq!(pp.validate_hgt(), false); 
	}

	#[test]
	fn day4_test8() {
		let pp = Passport::new(vec!["hgt:190"]);
		assert_eq!(pp.validate_hgt(), false); 
	}

	#[test]
	fn day4_test9() {
		let pp = Passport::new(vec!["hcl:#123abc"]);
		assert_eq!(pp.validate_hcl(), true); 
	}

	#[test]
	fn day4_test10() {
		let pp = Passport::new(vec!["hcl:#123abz"]);
		assert_eq!(pp.validate_hcl(), false); 
	}

	#[test]
	fn day4_test11() {
		let pp = Passport::new(vec!["hcl:123abc"]);
		assert_eq!(pp.validate_hcl(), false); 
	}

	#[test]
	fn day4_test12() {
		let pp = Passport::new(vec!["ecl:brn"]);
		assert_eq!(pp.validate_ecl(), true); 
	}

	#[test]
	fn day4_test13() {
		let pp = Passport::new(vec!["ecl:wat"]);
		assert_eq!(pp.validate_ecl(), false); 
	}

	#[test]
	fn day4_test14() {
		let pp = Passport::new(vec!["pid:000000001"]);
		assert_eq!(pp.validate_pid(), true); 
	}

	#[test]
	fn day4_test15() {
		let pp = Passport::new(vec!["pid:0123456789"]);
		assert_eq!(pp.validate_pid(), false); 
	}

	#[test]
	fn day4_test16() {
		let pp = Passport::new("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), false); 
	}

	#[test]
	fn day4_test17() {
		let pp = Passport::new("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm	ecl:grn pid:012533040 byr:1946"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), false); 
	}

	#[test]
	fn day4_test18() {
		let pp = Passport::new("hcl:dab227 iyr:2012	ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), false); 
	}

	#[test]
	fn day4_test19() {
		let pp = Passport::new("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), false); 
	}

	#[test]
	fn day4_test20() {
		let pp = Passport::new("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), true); 
	}

	#[test]
	fn day4_test21() {
		let pp = Passport::new("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), true); 
	}

	#[test]
	fn day4_test22() {
		let pp = Passport::new("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), true); 
	}

	#[test]
	fn day4_test23() {
		let pp = Passport::new("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
								.split(' ').collect::<Vec<&str>>());
		assert_eq!(pp.validate(), true); 
	}
}
