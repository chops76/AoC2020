use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    name: String,
    min1: i32,
    max1: i32,
    min2: i32,
    max2: i32
}

fn parse_rule(rule_str: &str) -> Rule {
    let split: Vec<&str> = rule_str.split(":").collect();
    let range_strs: Vec<&str> = split[1].split(" ").collect();
    let range1: Vec<&str> = range_strs[1].split("-").collect();
    let range2: Vec<&str> = range_strs[3].split("-").collect();

    Rule {
        name: split[0].to_string(),
        min1: range1[0].parse().unwrap(),
        max1: range1[1].parse().unwrap(),
        min2: range2[0].parse().unwrap(),
        max2: range2[1].parse().unwrap()
    }
}

fn parse_input(path: &str) -> (Vec<Rule>, Vec<i32>, Vec<Vec<i32>>) {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

    let split: Vec<&str> = fstr.split("\n\n").collect(); 
    let rules: Vec<Rule> = split[0].split("\n").map(|s| parse_rule(s)).collect();
    
    let yours: Vec<i32> = split[1].split("\n").skip(1).next().unwrap()
                                  .split(",").map(|s| s.parse()).flatten().collect();

    let theirs: Vec<Vec<i32>> = split[2].split("\n").skip(1)
                                   .map(|s| s.split(",").map(|s1| s1.parse()).flatten().collect()).collect();

    (rules, yours, theirs)
}

fn valid_num(num: i32, rules: &Vec<Rule>) -> bool {
    for rule in rules {
        if (num >= rule.min1 && num <= rule.max1) || 
             (num >= rule.min2 && num <= rule.max2) {
            return true;
        }
    }
    false
}

fn part1(rules: &Vec<Rule>, theirs: &Vec<Vec<i32>>) -> i32 {
    let nums = theirs.iter().flatten().collect::<Vec<&i32>>();
    nums.iter().map(|&&n| n).filter(|n| !valid_num(*n, rules)).sum()
}

fn valid_ticket(rules: &Vec<Rule>, vals: &Vec<i32>) -> bool {
    for val in vals {
        if !valid_num(*val, rules) {
            return false;
        }
    }
    true
}

fn part2(rules: &Vec<Rule>, theirs: &Vec<Vec<i32>>, yours: &Vec<i32>) -> i64 {
    let mut hs: Vec<Vec<bool>> = vec![vec![true; theirs[0].len()];rules.len()];
    let valid: Vec<&Vec<i32>> = theirs.iter().filter(|t| valid_ticket(rules, t)).collect();
    for ticket in valid {
        for (fieldnum, fieldval) in ticket.iter().enumerate() {
            for (rulenum, rule) in rules.iter().enumerate() {
                if !((*fieldval >= rule.min1 && *fieldval <= rule.max1) || 
                     (*fieldval >= rule.min2 && *fieldval <= rule.max2)) {
                    hs[rulenum][fieldnum] = false;
                }
            }
        }
    }
    let mut answermap: HashMap<String, i32> = HashMap::new();
    let mut changed = true;
    while changed == true {
        changed = false;
        for i in 0..hs.len() {
            if hs[i].iter().filter(|i| **i).count() == 1 {
                let pos = hs[i].iter().enumerate().filter(|(_,i)| **i).next().unwrap().0;
                answermap.insert(rules[i].name.clone(), pos as i32);
                for j in 0..hs.len() {
                    if j == i {
                        continue;
                    }
                    if hs[j][pos] == true {
                        changed = true;
                        hs[j][pos] = false;
                    }
                }
            }
        }
    }
    let dep_fields = answermap.iter().filter(|(key, _)|key.contains("departure")).map(|(_,val)|*val).collect::<Vec<i32>>();
    dep_fields.iter().map(|i| yours[*i as usize] as i64).product::<i64>()
}

pub fn main() {
    let (rules, yours, theirs) = parse_input("./input/day16/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&rules, &theirs);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&rules, &theirs, &yours);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day16_test1() {
        let (rules, yours, theirs) = parse_input("./input/day16/test.txt");
        assert_eq!(part1(&rules, &theirs),71);
    }

}
