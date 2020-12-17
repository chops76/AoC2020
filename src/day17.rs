use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = HashSet<Point>;
#[derive(Hash, Eq, PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64
}

fn parse_input(path: &str) -> Input {
    let mut hs = HashSet::new();
    let f = File::open(path).unwrap();
    for (y, x_str) in BufReader::new(f).lines().flatten().enumerate() {
        for(x, c) in x_str.chars().enumerate() {
            if c == '#' {
                hs.insert(Point {x: x as i64, y: y as i64, z: 0, w: 0});
            }
        }
    }

    hs
}

fn neighbors3d(p: &Point) -> HashSet<Point> {
    let mut ret_set = HashSet::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                ret_set.insert(Point{x:p.x + x, y:p.y + y, z:p.z + z, w: 0});
            }
        }
    }
    ret_set
}

fn neighbors4d(p: &Point) -> HashSet<Point> {
    let mut ret_set = HashSet::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    ret_set.insert(Point{x:p.x + x, y:p.y + y, z:p.z + z, w: p.w + w});
                }

            }
        }
    }
    ret_set
}

fn cycle(hs: &Input, part2: bool) -> HashSet<Point> {
    let mut to_check = hs.clone();

    for p in hs {
        let n = match part2 { true => neighbors4d(p), false => neighbors3d(p) };
        for nv in n {
            to_check.insert(nv);
        }
    }

    let mut new_set: HashSet<Point> = HashSet::new();
    for p in to_check {
        let num = match part2 {
            true => hs.intersection(&neighbors4d(&p)).count(),
            false => hs.intersection(&neighbors3d(&p)).count()};
        if hs.contains(&p) {
            if num == 2 || num == 3 {
                new_set.insert(p);
            } 
        } else {
            if num == 3 {
                new_set.insert(p);
            }
        }
    }
    new_set
}

fn part1(hs: &Input) -> u64 {
    let mut active = hs.to_owned();
    for _ in 0..6 {
        active = cycle(&active, false);
    }
    active.len() as u64
}

fn part2(hs: &Input) -> u64 {
    let mut active = hs.to_owned();
    for _ in 0..6 {
        active = cycle(&active, true);
    }
    active.len() as u64
}

pub fn main() {
    let hm = parse_input("./input/day17/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&hm);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&hm);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day17_test1() {
        let hm = parse_input("./input/day17/test.txt");
        assert_eq!(part1(&hm),112);
    }

    #[test]
    fn day17_test2() {
        let hm = parse_input("./input/day17/test.txt");
        assert_eq!(part2(&hm),848);
    }

}
