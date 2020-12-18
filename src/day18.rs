use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn apply_op(a: i64, b: i64, op: char) -> i64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        _ => a / b
    }
}

fn prec1(op: char) -> u32 {
    if op == '+' || op == '*' {
        return 1;
    }
    return 0
}

fn prec2(op: char) -> u32 {
    match op {
        '*' => 1,
        '+' => 2,
        _ => 0
    }
}

fn eval(tokens: &str, part2: bool) -> i64 {
    let mut values: Vec<i64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let expr: Vec<char> = tokens.chars().collect();
    let mut i = 0;
    while i < expr.len() { 
        if expr[i] == ' ' {
            i += 1;
            continue;
        } else if expr[i] == '(' {
            ops.push(expr[i]);
        }
         
        else if expr[i].is_digit(10) {
            let mut val = 0;
             
            while i < expr.len() && expr[i].is_digit(10) {
                val = (val*10) + expr[i].to_digit(10).unwrap() as i64;
                i += 1;
            }
             
            values.push(val);
            i-=1;
        }
         
        else if expr[i] == ')' {
            while ops.len() != 0 && ops[ops.len()-1] != '('
            {
                let val2 = values[values.len()-1];
                values.pop();
                 
                let val1 = values[values.len()-1];
                values.pop();
                 
                let op = ops[ops.len()-1];
                ops.pop();
                 
                values.push(apply_op(val1, val2, op));
            }
            
            if ops.len() != 0 {
               ops.pop();
            }
        }
         
        else
        {
            let prec = match part2 {
                true => prec2,
                false => prec1
            };

            while ops.len() != 0 && prec(ops[ops.len()-1]) >= prec(expr[i]) {
                let val2 = values[values.len()-1];
                values.pop();
                 
                let val1 = values[values.len()-1];
                values.pop();
                 
                let op = ops[ops.len()-1];
                ops.pop();
                 
                values.push(apply_op(val1, val2, op));
            }
             
            ops.push(expr[i]);
        }
        i+=1;
    }
     
    while ops.len() != 0 {
        let val2 = values[values.len()-1];
        values.pop();
                 
        let val1 = values[values.len()-1];
        values.pop();
                 
        let op = ops[ops.len()-1];
        ops.pop();
                 
        values.push(apply_op(val1, val2, op));
    }
     
    values[values.len()-1]
}

fn part1(input: &Input) -> i64 {
    input.iter().map(|s| eval(s, false)).sum()
}

fn part2(input: &Input) -> i64 {
    input.iter().map(|s| eval(s, true)).sum()
}

pub fn main() {
    let input = parse_input("./input/day18/input.txt");

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
    fn day18_test1() {
        assert_eq!(eval("2 * 3 + (4 * 5)", false),26);
    }

    #[test]
    fn day18_test2() {
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", false),437);
    }

    #[test]
    fn day18_test3() {
        assert_eq!(eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false),13632);
    }

    #[test]
    fn day18_test4() {
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false),12240);
    }

    #[test]
    fn day18_test5() {
        assert_eq!(eval("1 + 2 * 3 + 4 * 5 + 6", true),231);
    }

    #[test]
    fn day18_test6() {
        assert_eq!(eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true),23340);
    }  
}
