// Problem: https://adventofcode.com/2018/day/2#part2

use std::fs;

#[macro_use]
extern crate itertools;
fn main() {
    let contents = fs::read_to_string("data/day2.txt").expect("Error reading file");
    for (box_id_1, box_id_2) in iproduct!(contents.lines(), contents.lines()) {
        let (common_chars, _): (Vec<char>, Vec<char>) = box_id_1
            .chars()
            .zip(box_id_2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .unzip();
        if common_chars.len() == box_id_1.len() - 1 {
            let common_chars: String = common_chars.into_iter().collect();
            println!("Common Letters: {}", common_chars);
            break;
        }
    }
}
