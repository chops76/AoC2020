use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Entry {
    foods: HashSet<String>,
    allergens: HashSet<String>
}

type Input = Vec<Entry>;
type AllergenMap = HashMap<String, HashSet<String>>;

fn parse_line(s: &str) -> Entry {
    let spl: Vec<&str> = s.split(" (").collect();
    Entry {
        foods: spl[0].split(" ").map(|s| s.to_string()).collect(),
        allergens: spl[1].split(" ").skip(1).map(|s| s[..s.len()-1].to_string()).collect()
    }
}

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

    fstr.split("\n").map(|s| parse_line(s)).collect() 
}

fn process_entry(entry: &Entry, hm: &mut AllergenMap) {
    for allergen in &entry.allergens {
        if hm.contains_key(allergen) {
            let mut int_iter = hm[allergen].intersection(&entry.foods).map(|s| s.clone());
            hm.insert(allergen.to_string(), int_iter.collect());
        } else {
            hm.insert(allergen.to_string(), entry.foods.clone());
        }
    }
}

fn part1(input: &Input) -> u64 {
    let mut a_map: AllergenMap = HashMap::new();
    let mut seen_map: HashMap<String, u64> = HashMap::new();
    for entry in input {
        process_entry(&entry, &mut a_map);
    }
    for entry in input {
        for food in &entry.foods {
            if seen_map.contains_key(food) {
                seen_map.insert(food.to_string(), seen_map[food] + 1);
            } else {
                seen_map.insert(food.to_string(), 1);
            }
        }
    }
    for (_, foods) in &a_map {
        for food in foods {
            seen_map.insert(food.to_string(), 0);
        }
    }
    seen_map.iter().map(|(_, v)| v).sum()
}

fn part2(input: &Input) -> String {
    let mut a_map: AllergenMap = HashMap::new();
    let mut danger: Vec<(String, String)> = Vec::new();
    for entry in input {
        process_entry(&entry, &mut a_map);
    }

    let mut food_vec: Vec<(String, HashSet<String>)> = a_map.iter().map(|(a, b)| (a.to_string(), b.clone())).collect();
    let mut changed = true;
    while changed {
        changed = false; 
        for i in 0..food_vec.len() {
            if food_vec[i].1.len() == 1 {
                danger.push((food_vec[i].0.to_string(), food_vec[i].1.iter().next().unwrap().to_string()));
                changed = true;
                break;
            }
        }
        if changed {
            for i in 0..food_vec.len() {
                let mut tmp = food_vec[i].1.clone();
                tmp.remove(&danger[danger.len()-1].1);
                food_vec[i].1 = tmp;
            }
        }
    }
    
    danger.sort();
    let mut ret_str = String::new();
    
    for i in 0..danger.len() {
        if i != 0 {
            ret_str += ",";
        }
        ret_str += &danger[i].1;
    }
    
    ret_str
}

pub fn main() {
    let input = parse_input("./input/day21/input.txt");   
    
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
    fn day21_test1() {
        let input = parse_input("./input/day21/test.txt");   
        assert_eq!(part1(&input),5);
    }

    #[test]
    fn day21_test2() {
        let input = parse_input("./input/day21/test.txt");   
        assert_eq!(part2(&input),"mxmxvkd,sqjhc,fvjkl");
    }
}
