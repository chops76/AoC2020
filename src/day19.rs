use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum Rule {
    Rules(Vec<Vec<usize>>),
    Character(char)
}

type RuleList = Vec<Rule>;

fn parse_rule(rule_str: &str, rule_vec: &mut RuleList) {
    let rule_num: usize = rule_str.split(":").next().unwrap().parse().unwrap();
    let rule = &rule_str.split(":").skip(1).next().unwrap()[1..];
    let char_vec: Vec<char> = rule.chars().collect();
    if char_vec[0] == '\"' {
        rule_vec[rule_num] = Rule::Character(char_vec[1]);
        return;
    }
    let rules: Vec<&str> = rule.split(" | ").collect();
    let mut ret_vec = Vec::new();
    for r in rules {
        ret_vec.push(r.split(" ").map(|s| s.parse()).flatten().collect());
    }
    rule_vec[rule_num] = Rule::Rules(ret_vec);
}

fn parse_input(path: &str) -> (RuleList, Vec<String>) {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

    let split: Vec<&str> = fstr.split("\n\n").collect(); 
    let mut rules: RuleList = vec![Rule::Character('d');134];
    for rule in split[0].split("\n") {
        parse_rule(rule, &mut rules);
    }

    (rules, split[1].split("\n").map(|s| s.to_string()).collect())
}

fn recur(string: &str, rule: usize, rules: &RuleList, hm: &mut HashMap<(String, usize), bool>) -> bool {
    if hm.contains_key(&(string.to_string(), rule)) {
        return hm[&(string.to_string(), rule)];
    }
    match &rules[rule] {
        Rule::Character(c) => {
            if *c == string.chars().nth(0).unwrap() && string.len() == 1 {
                hm.insert((string.to_string(), rule), true);
                return true;
            }
            hm.insert((string.to_string(), rule), false);
            return false;
        }
        Rule::Rules(v) => {
            for rule_list in v {
                if rule_list.len() == 1 {
                    if recur(string, rule_list[0], rules, hm) == true {
                        hm.insert((string.to_string(), rule), true);
                        return true;
                    }
                } else if rule_list.len() == 2 {
                    for r in 1..string.len() {
                        if recur(&string[0..r], rule_list[0], rules, hm) && recur(&string[r..], rule_list[1], rules, hm) {
                            hm.insert((string.to_string(), rule), true);
                            return true;
                        }
                    }
                } else if string.len() > 2 {
                    for i in 1..string.len() - 1 {
                        for j in i..string.len() {
                            if recur(&string[0..i], rule_list[0], rules, hm) && 
                               recur(&string[i..j], rule_list[1], rules, hm) && 
                               recur(&string[j..], rule_list[2], rules, hm) {
                                hm.insert((string.to_string(), rule), true);
                                return true;
                            }                            
                        }
                    }
                }
            }
        }
    }
    hm.insert((string.to_string(), rule), false);
    false
}

fn part1(rules: &RuleList, strings: &Vec<String>) -> usize {
    let mut hm: HashMap<(String, usize), bool> = HashMap::new();
    strings.iter().filter(|s| recur(s, 0, &rules, &mut hm)).count()
}

fn part2(rules: &RuleList, strings: &Vec<String>) -> usize {
    let mut new_rules = rules.clone();
    parse_rule("8: 42 | 42 8", &mut new_rules);
    parse_rule("11: 42 31 | 42 11 31", &mut new_rules);
    let mut hm: HashMap<(String, usize), bool> = HashMap::new();
    strings.iter().filter(|s| recur(s, 0, &new_rules, &mut hm)).count()
}

pub fn main() {
    let (rules, strings) = parse_input("./input/day19/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&rules, &strings);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&rules, &strings);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day19_test1() {
        let (rules, strings) = parse_input("./input/day19/test2.txt");
        assert_eq!(part1(&rules, &strings),2);
    }

    #[test]
    fn day19_test2() {
        let (rules, strings) = parse_input("./input/day19/test1.txt");
        assert_eq!(part1(&rules, &strings),3);
    }

    #[test]
    fn day19_test3() {
        let (rules, strings) = parse_input("./input/day19/test1.txt");
        assert_eq!(part2(&rules, &strings),12);
    }
}
