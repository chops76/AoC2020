use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast
}

type Input = Vec<Vec<Direction>>;

fn parse_line(s: &str) -> Vec<Direction> {
    let mut ret_vec = Vec::new();
    let mut chars = s.chars();
    let mut c = chars.next();
    while c != None {
        match c.unwrap() {
            'e' => ret_vec.push(Direction::East),
            'w' => ret_vec.push(Direction::West),
            's' => match chars.next().unwrap() {
                'w' => ret_vec.push(Direction::SouthWest),
                _ => ret_vec.push(Direction::SouthEast)
            },
            _ => match chars.next().unwrap() {
                'w' => ret_vec.push(Direction::NorthWest),
                _ => ret_vec.push(Direction::NorthEast)
            },
        };
        c = chars.next();
    }
    ret_vec
}

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn init_tiles(input: &Input, hm: &mut HashMap<(i32, i32), bool>) {
    for path in input {
        let mut x:i32 = 0;
        let mut y:i32 = 0;
        
        for dir in path {
            if y.abs() % 2 == 0 {
                match dir {
                    Direction::West => x -= 1,
                    Direction::East => x += 1,
                    Direction::NorthWest => { x -= 1; y += 1 },
                    Direction::NorthEast => y += 1,
                    Direction::SouthEast => y -= 1,
                    Direction::SouthWest => { x -= 1; y -= 1 }
                }
            } else {
                match dir {
                    Direction::West => x -= 1,
                    Direction::East => x += 1,
                    Direction::NorthWest => y += 1,
                    Direction::NorthEast => { x += 1; y += 1},
                    Direction::SouthEast => { x += 1; y -= 1},
                    Direction::SouthWest => y -= 1
                }                
            }
        }
        let key = (x, y);
        if hm.contains_key(&key) {
            hm.insert(key, !hm[&key]);
        } else {
            hm.insert(key, true);
        }
    }
}

fn neighbors(n: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut hs = HashSet::new();
    let (x,y) = n;
    if y.abs() % 2 == 0 {
        hs.insert((x-1,y));
        hs.insert((x-1,y+1));
        hs.insert((x, y+1));
        hs.insert((x+1,y));
        hs.insert((x,y-1));
        hs.insert((x-1,y-1));
    } else {
        hs.insert((x-1,y));
        hs.insert((x,y+1));
        hs.insert((x+1, y+1));
        hs.insert((x+1,y));
        hs.insert((x+1,y-1));
        hs.insert((x,y-1));        
    }
    hs
}

fn next_day(hm: &mut HashMap<(i32, i32), bool>)
{
    let cur = hm.clone();
    let mut hs = HashSet::new();
    for i in cur.iter().filter(|(_, v)|**v).map(|v| v.0) {
        hs = hs.union(&neighbors(*i)).map(|n| *n).collect();
        hs.insert(*i);
    }
    for i in hs {
        let num_adj = neighbors(i).iter().filter(|p| cur.contains_key(p) && cur[&p]).count();
        
        if cur.contains_key(&i) && cur[&i] {
            //println!("Checking {:?}, currently black, {} adjacent", i, num_adj);
            if num_adj == 0 || num_adj > 2 {
                hm.insert(i, false);
            }
        } else {
            //println!("Checking {:?}, currently white, {} adjacent", i, num_adj);
            if num_adj == 2 {
                hm.insert(i, true);
            }
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut hm: HashMap<(i32, i32), bool> = HashMap::new();
    init_tiles(input, &mut hm);
    hm.iter().filter(|(_, v)|**v).count()
}

fn part2(input: &Input) -> usize {
    let mut hm: HashMap<(i32, i32), bool> = HashMap::new();
    init_tiles(input, &mut hm);
    for _ in 0..100 {
        next_day(&mut hm);
    }

    hm.iter().filter(|(_, v)|**v).count()
}

pub fn main() {
    let input = parse_input("./input/day24/test.txt");  
    
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
    fn day24_test1() { 
        assert_eq!(0,0);
    }
}
