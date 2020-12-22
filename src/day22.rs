use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Input = (VecDeque<u64>, VecDeque<u64>);

fn parse_player(s: &str) -> VecDeque<u64> {
    s.split("\n").skip(1).map(|s| s.parse().unwrap()).collect() 
}

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

    let mut spl = fstr.split("\n\n");
    (parse_player(spl.next().unwrap()), parse_player(spl.next().unwrap()))
}

fn score(p1: &VecDeque<u64>) -> u64 {
    p1.iter().rev().enumerate().map(|(a, b)| (a+1) as u64*b).sum()
}

fn p2_game(p1: &VecDeque<u64>, p2: &VecDeque<u64>, 
           hm: &mut HashMap<(VecDeque<u64>, VecDeque<u64>), (bool, u64)>) -> (bool, u64) {
    let key = (p1.clone(), p2.clone());
    if hm.contains_key(&key) {
        return hm[&key];
    }
    let mut hs: HashSet<(VecDeque<u64>, VecDeque<u64>)> = HashSet::new();
    let mut d1 = p1.clone();
    let mut d2 = p2.clone();
    while d1.len() != 0 && d2.len() != 0 {
        if hs.contains(&(d1.clone(), d2.clone())) {
            let ret_val = (true, score(&d1));
            hm.insert(key, ret_val);
            return ret_val;
        }
        hs.insert((d1.clone(), d2.clone()));
        let p1_wins;
        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();
        if d1.len() as u64 >= c1 && d2.len() as u64 >= c2 {
            let mut dc1 = d1.clone();
            dc1.truncate(c1 as usize);
            let mut dc2 = d2.clone();
            dc2.truncate(c2 as usize);
            let (winner, _) = p2_game(&dc1, &dc2, hm);
            p1_wins = winner;
        } else {
            p1_wins = c1 > c2;
        }
        if p1_wins {
            d1.push_back(c1);
            d1.push_back(c2);
        } else {
            d2.push_back(c2);
            d2.push_back(c1);
        }
    }
    let ret_val = match d1.len() == 0 { true => (false, score(&d2)), false => (true, score(&d1))};
    hm.insert(key, ret_val);
    return ret_val;
}

fn part2(p1: &VecDeque<u64>, p2: &VecDeque<u64>) -> u64 {
    let mut hs: HashMap<(VecDeque<u64>, VecDeque<u64>), (bool, u64)> = HashMap::new();
    let (_, score) = p2_game(p1, p2, &mut hs);
    score
}

fn part1(p1: &VecDeque<u64>, p2: &VecDeque<u64>) -> u64 {
    let mut d1 = p1.clone();
    let mut d2 = p2.clone();
    while d1.len() != 0 && d2.len() != 0 {
        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();
        if c1 > c2 {
            d1.push_back(c1);
            d1.push_back(c2);
        } else {
            d2.push_back(c2);
            d2.push_back(c1);
        }
    }
    let winner = match d1.len() == 0 { true => d2, false => d1 };
    score(&winner)
}

pub fn main() {
    let (p1, p2) = parse_input("./input/day22/test.txt");  
    
    let p1_timer = Instant::now();
    let p1_result = part1(&p1, &p2);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&p1, &p2);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day22_test1() {
        let (p1, p2) = parse_input("./input/day22/test.txt");  
        assert_eq!(part1(&p1, &p2),306);
    }

    #[test]
    fn day22_test2() {
        let (p1, p2) = parse_input("./input/day22/test.txt");  
        assert_eq!(part2(&p1, &p2),291);
    }

    #[test]
    fn day22_test3() {
        let (p1, p2) = parse_input("./input/day22/test2.txt");  
        assert_eq!(part2(&p1, &p2),105);
    }
}
