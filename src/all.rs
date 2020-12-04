mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
	let days = [
		day1::main,
		day2::main,
		day3::main,
		day4::main
	];

	for (day, main) in days.iter().enumerate() {
		println!( "--------------------------" );
		println!( "Day {}",	day + 1	);
		println!( "--------------------------" );
		main();
		println!();
	}
}
