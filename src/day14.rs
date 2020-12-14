use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::time::Instant;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn get_masks(mask: &str) -> (u64, u64)
{
    let or_mask: String = mask.chars().map(|c| match c { '1' => '1', _ => '0'}).collect();
    let and_mask: String = mask.chars().map(|c| match c { '0' => '0', _ => '1'}).collect();
    (u64::from_str_radix(&and_mask, 2).unwrap(), u64::from_str_radix(&or_mask, 2).unwrap())
}

fn part1(input: &Input) -> u64 {
    let mut and_mask: u64 = 0;
    let mut or_mask: u64 = 0;
    let mut hs = HashMap::new();
    for cmd in input {
        let spl: Vec<&str> = cmd.split(" = ").collect();
        if spl[0] == "mask" {
            let (new_and, new_or) = get_masks(spl[1]);
            and_mask = new_and;
            or_mask = new_or;
        } else {
            let addr:u64 = spl[0][4..spl[0].len()-1].parse().unwrap();
            let mut val:u64 = spl[1].parse().unwrap();
            val &= and_mask;
            val |= or_mask;
            hs.insert(addr, val);
        }
    }
    hs.iter().map(|(_, b)| b).sum()
}

fn get_addr_string(addr_str: &str, mask: &str, val: u64) -> String {
    let mut char_vec: Vec<char> = addr_str.chars().rev().collect();
    let mut count = 0;
    for c in mask.chars().rev().enumerate() {
        if c.1 == 'X' {
            char_vec[c.0] = match (val >> count) & 0x01 {
                0 => '0',
                _ => '1'
            };
            count += 1;
        }
    }

    char_vec.iter().rev().collect()
}

fn part2(input: &Input) -> u64 {
    let mut hs = HashMap::new();
    let mut mask = String::new();
    let mut or_mask: u64 = 0;
    let mut num_addrs = 0;
    for cmd in input {
        let spl: Vec<&str> = cmd.split(" = ").collect();
        if spl[0] == "mask" {
            mask = spl[1].to_owned();
            let (_, new_or) = get_masks(spl[1]);
            or_mask = new_or;
            num_addrs = mask.chars().filter(|c| *c == 'X').count();
        } else {
            let mut addr: u64 = spl[0][4..spl[0].len()-1].parse().unwrap();
            addr |= or_mask;
            
            for i in 0..(u64::pow(2,num_addrs as u32)) {
                let this_addr = get_addr_string(&format!("{:036b}", addr), &mask, i);
                let addr:u64 = u64::from_str_radix(&this_addr, 2).unwrap();
                let val:u64 = spl[1].parse().unwrap();
                hs.insert(addr, val);
            }
        }
    }
    hs.iter().map(|(_, b)| b).sum()
}

pub fn main() {
    let input = parse_input("./input/day14/input.txt");

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
    fn day14_test1() {
        let input = parse_input("./input/day14/test.txt");
        assert_eq!(part1(&input),165);
    }

    #[test]
    fn day14_test2() {
        let input = parse_input("./input/day14/test2.txt");
        assert_eq!(part2(&input),208);
    }
}
