mod day1;

fn main() {
	let days = [
		day1::main,
	];

	for (day, main) in days.iter().enumerate() {
		println!( "--------------------------" );
		println!( "Day {}",	day + 1	);
		println!( "--------------------------" );
		main();
		println!();
	}
}
