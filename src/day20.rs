use std::io::Read;
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
#[derive(Clone)]
struct Tile {
    id: u32,
    pic: Vec<Vec<bool>>,
    flipped: Vec<Vec<bool>>
}

#[derive(Debug)]
struct Sides {
    id: u32,
    sides: Vec<u16>,
    flipped: Vec<u16>
}

#[derive(Debug)]
struct Pos {
    idx: usize,
    is_flipped: bool,
    rotation: usize
}

impl Sides {
    pub fn top(&self, is_flipped: bool, rotation: usize) -> u16 {
        match is_flipped {
            true=>self.flipped[(4 - rotation) % 4],
            false=>self.sides[(4 - rotation) % 4]
        }
    }
    pub fn right(&self, is_flipped: bool, rotation: usize) -> u16 {
        match is_flipped {
            true=>self.flipped[(5 - rotation) % 4],
            false=>self.sides[(5 - rotation) % 4]
        }
    }

    pub fn bottom(&self, is_flipped: bool, rotation: usize) -> u16 {
        match is_flipped {
            true=>self.flipped[(6 - rotation) % 4],
            false=>self.sides[(6 - rotation) % 4]
        }
    }

    pub fn left(&self, is_flipped: bool, rotation: usize) -> u16 {
        match is_flipped {
            true=>self.flipped[(7 - rotation) % 4],
            false=>self.sides[(7 - rotation) % 4]
        }
    }
}

type Input = Vec<Tile>;

fn parse_line(s: &str) -> Vec<bool> {
    s.bytes().map(|b| b == b'#').collect()
}

fn parse_tile(tile_str: &str) -> Tile {
    let id_str = tile_str.split("\n").next().unwrap().split(" ").skip(1).next().unwrap();
    let grid_pic:Vec<Vec<bool>> = tile_str.split("\n").skip(1).map(|s| parse_line(s)).collect();
    let tmp = grid_pic.clone();
    let mut flip:Vec<Vec<bool>> = Vec::new(); 
    for line in tmp {
        flip.push(line.iter().rev().map(|b| *b).collect());
    }
    let id = id_str[0..id_str.len()-1].parse().unwrap();
    Tile {
        id: id,
        pic: grid_pic,
        flipped: flip
    }
    
}

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();

    fstr.split("\n\n").map(|s| parse_tile(s)).collect() 
}

fn calc_sides(tile: &Tile) -> Sides {
    let mut tmp_vec = Vec::new();
    let mut flipped_vec = Vec::new();
    let top_str = tile.pic[0].iter().map(|b| match b { true => '1', false => '0' })
                 .collect::<String>();
    let right_str = (0..tile.pic.len()).map(|n| tile.pic[n][tile.pic[n].len()-1])
                    .map(|b| match b { true => '1', false => '0' }).collect::<String>();
    let bot_str = tile.pic[tile.pic.len()-1].iter().rev().map(|b| match b { true => '1', false => '0' })
                 .collect::<String>();
    let left_str = (0..tile.pic.len()).rev().map(|n| tile.pic[n][0])
                 .map(|b| match b { true => '1', false => '0' }).collect::<String>();
    tmp_vec.push(u16::from_str_radix(&top_str, 2).unwrap());
    tmp_vec.push(u16::from_str_radix(&right_str, 2).unwrap());
    tmp_vec.push(u16::from_str_radix(&bot_str, 2).unwrap());
    tmp_vec.push(u16::from_str_radix(&left_str, 2).unwrap());
    flipped_vec.push(u16::from_str_radix(&top_str.chars().rev().collect::<String>(), 2).unwrap());
    flipped_vec.push(u16::from_str_radix(&left_str.chars().rev().collect::<String>(), 2).unwrap());
    flipped_vec.push(u16::from_str_radix(&bot_str.chars().rev().collect::<String>(), 2).unwrap());
    flipped_vec.push(u16::from_str_radix(&right_str.chars().rev().collect::<String>(), 2).unwrap());
    Sides {
        id: tile.id,
        sides: tmp_vec,
        flipped: flipped_vec
    }
}

fn part1(input: &Input) -> u64 {
    let side_vec: Vec<Sides> = input.iter().map(|t| calc_sides(t)).collect();
    let mut ret_val = 1;
    for i in 0..side_vec.len() {
        let mut matched = 0;
        for idx in 0..side_vec[i].sides.len() {
            for j in 0..side_vec.len() {
                if i == j {
                    continue;
                }
                if side_vec[j].sides.contains(&side_vec[i].sides[idx]) {
                    matched += 1;
                    break;
                }
                if side_vec[j].flipped.contains(&side_vec[i].sides[idx]) {
                    matched += 1;
                    break;
                }
            }
        }
        if matched == 2 {
            ret_val *= side_vec[i].id as u64;

        }
    }
    ret_val
}

fn find_match(side_val: u16, side_id: u32, side_vec: &Vec<Sides>) -> (u32, usize, bool)
{
    for j in 0..side_vec.len() {
        if side_vec[j].id == side_id {
            continue;
        }
        if side_vec[j].sides.contains(&side_val) {
            return (side_vec[j].id, j, true);
        }
        if side_vec[j].flipped.contains(&side_val) {
            return (side_vec[j].id, j, false);
        }
    }
    (0, 0, false)
}

fn matches(side1: u16, side2: u16) -> bool
{
    let rev_str = format!("{:010b}", side2).chars().rev().collect::<String>();
    side1 == u16::from_str_radix(&rev_str, 2).unwrap()
}

fn part2(input: &Input) -> u64 {
    let mut side_vec: Vec<Sides> = input.iter().map(|t| calc_sides(t)).collect();
    let mut ret_val = 1;
    let mut top_left = 0;
    for i in 0..side_vec.len() {
        let mut matched = 0;
        for idx in 0..side_vec[i].sides.len() {
            for j in 0..side_vec.len() {
                if i == j {
                    continue;
                }
                if side_vec[j].sides.contains(&side_vec[i].sides[idx]) {
                    matched += 1;
                    break;
                }
                if side_vec[j].flipped.contains(&side_vec[i].sides[idx]) {
                    matched += 1;
                    break;
                }
            }
        }
        if matched == 2 {
            top_left = i;
            break;
        }
    }

    let mut grid = Vec::new();
    let mut top_row = Vec::new();
    // Rotate top left
    let mut tl_rot = 0;
    loop {
        let (left_id, _, _) = find_match(side_vec[top_left].left(false, tl_rot), side_vec[top_left].id, &side_vec);
        let (top_id, _, _) = find_match(side_vec[top_left].top(false, tl_rot), side_vec[top_left].id, &side_vec);
        if left_id == 0 && top_id == 0 {
            break;
        }
        tl_rot+=1;
    }

    top_row.push(Pos {idx: top_left, rotation: tl_rot, is_flipped: false});
    loop {
        let prev = &top_row[top_row.len()-1];
        let prev_right = side_vec[prev.idx].right(prev.is_flipped, prev.rotation);
        let (id, idx, flipped) = find_match(prev_right, side_vec[prev.idx].id, &side_vec);
        if id == 0 {
            break;
        }
        let mut new_rot = 0;
        while !matches(prev_right, side_vec[idx].left(flipped, new_rot)) {
            new_rot+=1;
        }
        top_row.push(Pos {idx: idx, rotation: new_rot, is_flipped: flipped});
    }
    
    grid.push(top_row);
    loop {
        let above = &grid[grid.len()-1][0];
        let (id, _, _) = find_match(side_vec[above.idx].bottom(above.is_flipped, above.rotation),
                                    side_vec[above.idx].id, &side_vec);
        if id == 0 {
            break;
        }
        let mut cur_row = Vec::new();
        for i in 0..grid[0].len() {
            let above = &grid[grid.len()-1][i];
            let above_bot = side_vec[above.idx].bottom(above.is_flipped, above.rotation);
            let (id, idx, flipped) = find_match(above_bot, side_vec[above.idx].id, &side_vec); 

            let mut new_rot = 0;
            while !matches(above_bot,side_vec[idx].top(flipped, new_rot)) {
                new_rot+=1;
            }     
            cur_row.push(Pos {idx: idx, rotation: new_rot, is_flipped: flipped});      
        }
        grid.push(cur_row);
    }
    let dim = grid.len();
    let mut picture = vec!(vec!(false;8*dim);8*dim);
    for i in 0..dim {
        for j in 0..dim {
            let index = grid[i][j].idx;
            let rot = grid[i][j].rotation;
            let flipped = grid[i][j].is_flipped;
            for x in 1..9 {
                for y in 1..9 {
                    let (x_pos, y_pos) = trans_coord(x, y, rot, 9);
                    if flipped {
                        picture[i * 8 + (y-1)][j * 8 + (x-1)] = input[index].flipped[y_pos][x_pos];
                    } else {
                        picture[i * 8 + (y-1)][j * 8 + (x-1)] = input[index].pic[y_pos][x_pos];
                    }
                }
            }
        }
    }
    let size = picture.len();
    let mut flip:Vec<Vec<bool>> = Vec::new(); 
    for line in picture {
        flip.push(line.iter().rev().map(|b| *b).collect());
    }
    let mut picture2 = vec!(vec!(false;8*dim);8*dim);
    for x in 0..flip[0].len() {
        for y in 0..flip.len() {
            let (x_pos, y_pos) = trans_coord(x, y, 1, flip.len() - 1);
            picture2[x][y] = flip[x_pos][y_pos];
        }
    }
    let mut spots = Vec::new();
    for y in 0..size - 2 {
        for x in 0..size - 19 {
            if picture2[y+0][x+18] && picture2[y+1][x+0] && picture2[y+1][x+5] && picture2[y+1][x+6] &&
               picture2[y+1][x+11] && picture2[y+1][x+12] && picture2[y+1][x+17] && picture2[y+1][x+18] &&
               picture2[y+1][x+19] && picture2[y+2][x+1] && picture2[y+2][x+4] && picture2[y+2][x+7] &&
               picture2[y+2][x+10] && picture2[y+2][x+13] && picture2[y+2][x+16] {
                   spots.push((x, y));
               }
        }
    }
    for spot in spots {
        let x = spot.0;
        let y = spot.1;
        picture2[y+0][x+18] = false;
        picture2[y+1][x+0] = false;
        picture2[y+1][x+5] = false; 
        picture2[y+1][x+6] = false;
        picture2[y+1][x+11] = false;
        picture2[y+1][x+12] = false;
        picture2[y+1][x+17] = false;
        picture2[y+1][x+18] = false;
        picture2[y+1][x+19] = false;
        picture2[y+2][x+1] = false;
        picture2[y+2][x+4] = false;
        picture2[y+2][x+7] = false;
        picture2[y+2][x+10] = false;
        picture2[y+2][x+13] = false;
        picture2[y+2][x+16] = false;
    }

    let mut count = 0;
    for x in 0..size {
        for y in 0..size {
            if picture2[x][y] {
                count += 1;
            }
        }
    }

    count
}

fn trans_coord(x: usize, y: usize, rot: usize, dim: usize) -> (usize, usize)
{
    let mut x_pos = x;
    let mut y_pos = y;
    for _ in 0..rot {
        let tmp = x_pos;
        x_pos = y_pos;
        y_pos = tmp;
        y_pos = dim - y_pos;
    }
    (x_pos, y_pos)
}

pub fn main() {
    let input = parse_input("./input/day20/input.txt");   
    
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
    fn day20_test1() {
        assert_eq!(0,0);
    }
}
