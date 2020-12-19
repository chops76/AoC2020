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
 //   println!("Recur string = {}, rule = {}", string, rule);
    if hm.contains_key(&(string.to_string(), rule)) {
        //println!("Hash hit on {} rule {}", string, rule);
        return hm[&(string.to_string(), rule)];
    }
    match &rules[rule] {
        Rule::Character(c) => {
            if *c == string.chars().nth(0).unwrap() && string.len() == 1 {
//                println!("Returning true");
                hm.insert((string.to_string(), rule), true);
                return true;
            }
            hm.insert((string.to_string(), rule), false);
            return false;
        }
        Rule::Rules(v) => {
            /*
            if string.len() < 2 {
                println!("Bailing here");
                hm.insert((string.to_string(), rule), false);
                return false;
            }
            */
            for rule_list in v {
//                println!("Got a rule list {:?}", rule_list);
                if rule_list.len() > 2 {
//                    println!("BAD RULE LIST");
                    hm.insert((string.to_string(), rule), false);
                    return false;
                }
                if rule_list.len() == 1 {
//                    println!("In here");
                    for rl in rule_list {
                        if recur(string, *rl, rules, hm) == true {
                            hm.insert((string.to_string(), rule), true);
                            return true;
                        }
                    }
                    //hm.insert((string.to_string(), rule), false);
                    //return false;
                    //return recur(string, rule_list[0], rules, hm);
                } else {
                    for r in 1..string.len() {
                        if recur(&string[0..r], rule_list[0], rules, hm) && recur(&string[r..], rule_list[1], rules, hm) {
                            hm.insert((string.to_string(), rule), true);
                            return true;
                        }
                    }
                }
            }
        }
    }
    hm.insert((string.to_string(), rule), false);
    false
}

fn build(rules: &RuleList) {
    let mut cur: Vec<usize>;
    match &rules[0] {
        Rule::Rules(v) => cur = v[0].clone(),
        _ => return
    };
    
    let mut work_vec = Vec::new();
    work_vec.push(cur);
    while work_vec.len() != 0
    {
        cur = work_vec.pop().unwrap();
        println!("Num items = {}  Cur = {:?}", work_vec.len(), cur);
        let mut changed = false;
            
            for i in 0..cur.len() {
                if cur[i] == 41 || cur[i] == 48 {
//                if cur[i] == 4 || cur[i] == 5 {
                    continue;
                }
                match &rules[cur[i]] {
                    Rule::Rules(v) => {
                        for r in v {
                            let mut new_cur = Vec::new();
                            new_cur.extend_from_slice(&cur[..i]);
                            new_cur.extend(r.iter());
                            new_cur.extend_from_slice(&cur[i+1..]); 
                            work_vec.push(new_cur);
                        }
                    },
                    _ => { println!("PROBLEM"); println!("{:?}", cur); return }
                }
                changed = true;
                break;
            }
        
            if !changed {
            let str:String = cur.iter().map(|r| match r { 41 => 'a', _ => 'b'}).collect();
            //let str:String = cur.iter().map(|r| match r { 4 => 'a', _ => 'b'}).collect();
            let mut hm: HashMap<(String, usize), bool> = HashMap::new();
            if recur(&str, 0, rules, &mut hm) {
                println!("PASS: {}", str);
            } else {
                println!("FAIL: {}", str);
                return;
            }
        }
    }   
}

fn part1(rules: &RuleList, strings: &Vec<String>) -> u64 {
    let mut hm: HashMap<(String, usize), bool> = HashMap::new();
    let mut count = 0;
    for str in strings {
        if recur(str, 0, &rules, &mut hm) {
            println!("PASS: {}", str);
            count += 1;
        } else {
            println!("FAIL: {}", str);
        }
    }
    
    count
}

pub fn main() {
    let (rules, strings) = parse_input("./input/day19/input.txt");
/*
    for i in 0..rules.len() {
        println!("{}: {:?}", i, rules[i]);
    }
    */
    let mut hm: HashMap<(String, usize), bool> = HashMap::new();
 //   println!("{}", recur(&strings[0],0,&rules,&mut hm));
//    println!("{}", recur("bbbaaabbbbbaaabbabaabbaa",0,&rules,&mut hm));
//    println!("{}", recur("bbbaaabb",42,&rules,&mut hm));
    //println!("{}", recur("a",41,&rules,&mut hm));
//    println!("{}", recur("a",67,&rules,&mut hm));
 //   println!("{}", recur("bbbaaabbabaabbaa",11,&rules,&mut hm));

 //       build(&rules);

    
    let p1_timer = Instant::now();
    let p1_result = part1(&rules, &strings);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);
   /*

    let p2_timer = Instant::now();
    let p2_result = part2(&input);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
    */
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day19_test1() {
        let mut hm: HashMap<(String, usize), bool> = HashMap::new();
        let (rules, strings) = parse_input("./input/day19/input.txt");
        assert_eq!(recur(&strings[0],0,&rules,&mut hm),true);
    }
}
