// Problem: https://adventofcode.com/2018/day/3#part2

use std::fs;

struct Claim {
    id: String,
    min_x: usize, // usize so array access works
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

fn main() {
    let contents = fs::read_to_string("data/day3.txt").expect("Error reading file");
    let mut fabric = [[0; 1000]; 1000];
    let mut claims: Vec<Claim> = Vec::new();
    for line in contents.lines() {
        let claim = parse_line(line);
        for x in claim.min_x..=claim.max_x {
            for y in claim.min_y..=claim.max_y {
                fabric[x][y] += 1;
            }
        }
        claims.push(claim);
    }
    'outer: for claim in claims.into_iter() {
        for x in claim.min_x..=claim.max_x {
            for y in claim.min_y..=claim.max_y {
                if fabric[x][y] != 1 {
                    continue 'outer;
                }
            }
        }
        println!("Non-overlapping Claim ID: {}", claim.id);
        return;
    }
}

fn parse_line(line: &str) -> Claim {
    let line_parts: Vec<_> = line.split_whitespace().collect();
    let starting_position: Vec<_> = line_parts[2].split(',').collect();
    let rect_area: Vec<_> = line_parts[3].split('x').collect();
    let left: usize = starting_position[0].parse().unwrap();
    let top: usize = starting_position[1][..starting_position[1].len() - 1]
        .parse()
        .unwrap();
    let width: usize = rect_area[0].parse().unwrap();
    let height: usize = rect_area[1].parse().unwrap();
    Claim {
        id: line_parts[0][1..].to_string(),
        min_x: left + 1,
        min_y: top + 1,
        max_x: width + left,
        max_y: height + top,
    }
}
