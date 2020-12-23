use std::io::Read;
use std::fs::File;
use std::time::Instant;

type Input = Vec<u32>;

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.chars().map(|c| c.to_digit(10)).flatten().collect()
}

fn init_vector(input: &Vec<u32>, len: usize) -> Vec<usize> {
    let mut nodes: Vec<usize> = (1..len+2).collect();

    for idx in 0..input.len() - 1 {
        let value = input[idx] as usize;
        let next = input[idx + 1] as usize;

        nodes[value] = next;
    }

    nodes[0] = input[0] as usize;

    nodes[input[input.len() - 1] as usize] = match input.len() == len {
        true => input[0] as usize,
        false => input.len() + 1
    };

    if nodes.len() > input.len() + 1 {
        let idx = nodes.len() - 1;
        nodes[idx] = input[0] as usize;
    }

    nodes
}

fn do_game(cups: &mut Vec<usize>, rounds: u64) {
    for _ in 0..rounds {
        let head = cups[0];

        let n1 = cups[head];
        let n2 = cups[n1];
        let n3 = cups[n2];
        cups[head] = cups[n3];
        cups[0] = cups[n3];

        let mut dest = head - 1;
        while dest == n1 || dest == n2 || dest == n3 || dest == 0 {
            dest = match dest == 0 {
                true => cups.len()-1,
                false => dest - 1
            }
        }

        let unlink = cups[dest];
        cups[dest] = n1;
        cups[n3] = unlink;
    }
}

fn part2(input: &Input) -> u64 {
    let mut cups = init_vector(input, 1_000_000);
    do_game(&mut cups, 10_000_000);

    let a = cups[1];
    let b = cups[a];

    a as u64 * b as u64 
}

fn part1(input: &Input) -> String {
    let mut cups = init_vector(input, input.len());
    do_game(&mut cups, 100);
    let mut cur = cups[1];
    let mut ret_str = String::new();
    for _ in 0..input.len()-1 {
        ret_str += &(cur.to_string());
        cur = cups[cur];
    }
    
    ret_str
}

pub fn main() {
    let input = parse_input("./input/day23/input.txt");  
    
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
    fn day23_test1() {
        let input = parse_input("./input/day23/test.txt");  
        assert_eq!(part1(&input),"67384529");
    }

    #[test]
    fn day23_test2() {
        let input = parse_input("./input/day23/test.txt");  
        assert_eq!(part2(&input),149245887792);
    }
}
