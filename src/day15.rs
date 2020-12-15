use std::fs::File;
use std::io::Read;
use std::time::Instant;

type Input = Vec<u64>;

fn parse_input(path: &str) -> Input {
	let mut fstr = String::new();
	File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

	fstr.split(",").map(|s| s.parse()).flatten().collect() 
}

pub fn part1(input: &Input) -> u64 {
    let mut cache:Vec<i32> = vec![-1;30000000];
    for i in 0..input.len()-1 {
        cache[input[i] as usize] = (i + 1) as i32;
    }
    let mut last = input[input.len()-1];
    let mut count = input.len();
    while count < 2020 {
        let spot = cache[last as usize];
        cache[last as usize] = count as i32;
        if spot == -1 {
            last = 0;
        } else {
            last = (count - spot as usize) as u64;
        }
        count += 1;
    }
    last
}

pub fn part2(input: &Input) -> u64 {
    let mut cache:Vec<i32> = vec![-1;30000000];
    for i in 0..input.len()-1 {
        cache[input[i] as usize] = (i + 1) as i32;
    }
    let mut last = input[input.len()-1];
    let mut count = input.len();
    while count < 30000000 {
        let spot = cache[last as usize];
        cache[last as usize] = count as i32;
        if spot == -1 {
            last = 0;
        } else {
            last = (count - spot as usize) as u64;
        }
        count += 1;
    }
    last
}

pub fn main() {
    let input = parse_input("./input/day15/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&input);
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
    fn day15_test1() {
        let input = parse_input("./input/day15/test.txt");
        assert_eq!(part1(&input),436);
    }

    #[test]
    fn day15_test2() {
        let input = parse_input("./input/day15/test.txt");
        assert_eq!(part2(&input),175594);
    }
}
