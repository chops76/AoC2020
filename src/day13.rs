use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
struct Entry {
    offset: i64,
    val: i64
}

type Input = Vec<Entry>;

fn parse_input(path: &str) -> (i64, Input) {
    let f = File::open(path).unwrap();
    let mut lines = BufReader::new(f).lines().flatten();
    let target: i64 = lines.next().unwrap().parse().unwrap();
    let input = lines.next().unwrap().split(",").enumerate().filter(|(_, s)| *s != "x")
                     .map(|(a, s)| (a, s.parse().unwrap()))
                     .map(|(a, b)| Entry {offset: b - a as i64, val: b}).collect();
    (target, input)
}

fn mod_inv(a: i64, module: i64) -> i64 {
    let mut mn = (module, a);
    let mut xy = (0, 1);
   
    while mn.1 != 0 {
      xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
      mn = (mn.1, mn.0 % mn.1);
    }
   
    while xy.0 < 0 {
      xy.0 += module;
    }
    xy.0
  }

fn part1(target: i64, input: &Input) -> i64 {
    let mut cur_targ = target;
    loop {
        for i in input {
            if cur_targ % i.val == 0 {
                return i.val * (cur_targ - target);
            }
        }
        cur_targ += 1;
    }
}

fn part2(input: &Input) -> i64 {
    let n:i64 = input.iter().map(|e| e.val).product();
    input.iter().map(|i| i.offset * n/i.val * mod_inv(n/i.val, i.val)).sum::<i64>() % n
}

pub fn main() {
    let (target, input) = parse_input("./input/day13/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(target, &input);
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
    fn day13_test1() {
        let (target, input) = parse_input("./input/day13/test.txt");
        assert_eq!(part1(target, &input),295);
    }

    #[test]
    fn day13_test2() {
        let (_, input) = parse_input("./input/day13/test.txt");
        assert_eq!(part2(&input),1068781);
    }

    #[test]
    fn day13_test3() {
        let (_, input) = parse_input("./input/day13/test2.txt");
        assert_eq!(part2(&input),202);
    }

    #[test]
    fn day13_test4() {
        let (_, input) = parse_input("./input/day13/test3.txt");
        assert_eq!(part2(&input),3417);
    }
}
