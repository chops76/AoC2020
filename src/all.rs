mod day1;
mod day2;

fn main() {
	let days = [
		day1::main,
		day2::main,
	];

	for (day, main) in days.iter().enumerate() {
		println!( "--------------------------" );
		println!( "Day {}",	day + 1	);
		println!( "--------------------------" );
		main();
		println!();
	}
}
