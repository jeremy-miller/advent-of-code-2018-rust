// Problem: https://adventofcode.com/2018/day/2

use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day2.txt").expect("Error reading file");
    let mut twos = 0;
    let mut threes = 0;
    for line in contents.lines() {
        let mut char_counts = HashMap::new();
        for c in line.chars() {
            let char_count = char_counts.entry(c).or_insert(0);
            *char_count += 1;
        }
        if char_counts.iter().filter(|&(_, v)| *v == 2).count() > 0 {
            twos += 1;
        }
        if char_counts.iter().filter(|&(_, v)| *v == 3).count() > 0 {
            threes += 1;
        }
    }
    println!("Checksum: {}", twos * threes);
}
