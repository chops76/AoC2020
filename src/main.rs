use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod passport;

fn main() {
	let days = [
		day1::main,
		day2::main,
		day3::main,
		day4::main,
		day5::main,
		day6::main,
		day7::main,
		day8::main,
		day9::main,
		day10::main,
		day11::main,
		day12::main,
		day13::main,
		day14::main,
		day15::main,
	];

	let args: Vec<String> = env::args().collect();
	
	if args.len() > 1 {
		days[args[1].parse::<usize>().unwrap()-1]();
	} else {
		for (day, main) in days.iter().enumerate() {
			println!( "--------------------------" );
			println!( "Day {}",	day + 1	);
			println!( "--------------------------" );
			main();
			println!();
		}
	}
}
