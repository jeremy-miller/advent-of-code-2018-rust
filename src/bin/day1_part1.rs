// Problem: https://adventofcode.com/2018/day/1

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("data/day1.txt").expect("Error opening file");
    let reader = BufReader::new(f);
    let mut frequency = 0;
    for line in reader.lines() {
        update_frequency(line.unwrap(), &mut frequency);
    }
    println!("Resulting Frequency: {}", frequency);
}

fn update_frequency(line: String, frequency: &mut i32) {
    let new_frequency = line[1..].parse::<i32>().unwrap();
    if line.starts_with('+') {
        *frequency += new_frequency;
    } else {
        *frequency -= new_frequency;
    }
}
