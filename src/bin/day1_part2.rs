// Problem: https://adventofcode.com/2018/day/1#part2

use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day1.txt").expect("Error reading file");
    let mut frequency = 0;
    let mut frequencies_seen = HashSet::new();
    frequencies_seen.insert(0); // add starting frequency
    'outer: loop {
        // need to process the file multiple times to find duplicate
        for line in contents.lines() {
            update_frequency(line, &mut frequency);
            if frequencies_seen.contains(&frequency) {
                println!("First Duplicate Frequency: {}", frequency);
                break 'outer;
            } else {
                frequencies_seen.insert(frequency);
            }
        }
    }
}

fn update_frequency(line: &str, frequency: &mut i32) {
    let new_frequency = line[1..].parse::<i32>().unwrap();
    if line.starts_with('+') {
        *frequency += new_frequency;
    } else {
        *frequency -= new_frequency;
    }
}
