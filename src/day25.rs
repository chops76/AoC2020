use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::time::Instant;
use std::io::{BufRead, BufReader};

type Input = (u64,u64);

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    let num_vec:Vec<u64> = BufReader::new(f).lines().flatten().map(|s| s.parse().unwrap()).collect();
    (num_vec[0], num_vec[1])
}

fn get_loop(key: u64) -> u64 {
    let mut cur: u64 = 1;
    let mut count = 0;
    loop {
        count += 1;
        cur *= 7;
        cur %= 20201227;
        if cur == key {
            break;
        }
    }
    count
}

fn part1(input: (u64, u64)) -> u64 {
    let (card, door) = input;
    let door_loop = get_loop(door);
    let mut cur = 1;
    for _ in 0..door_loop {
        cur *= card;
        cur %= 20201227;
    }
    cur
}

pub fn main() {
    let input = parse_input("./input/day25/input.txt");  
    println!("{:?}", input);
    
    let p1_timer = Instant::now();
    let p1_result = part1(input);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day25_test1() { 
        let input = parse_input("./input/day25/test.txt");  
        assert_eq!(part1(input),14897079);
    }
}
