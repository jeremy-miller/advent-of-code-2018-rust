// Problem: https://adventofcode.com/2018/day/4

use chrono::NaiveDateTime;
use std::fs;

struct Observation {
    date_time: NaiveDateTime,
    guard_id: String,
    asleep: bool,
    awake: bool,
}

impl Default for Observation {
    fn default() -> Observation {
        Observation {
            date_time: NaiveDateTime::parse_from_str("1518-01-01 00:00", "%Y-%m-%d %H:%M").unwrap(),
            guard_id: "".to_string(),
            asleep: false,
            awake: false,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("data/day4.txt").expect("Error reading file");
    let mut observations: Vec<Observation> = Vec::new();
    for line in contents.lines() {
        let observation = parse_line(line);
        observations.push(observation);
    }
    println!("Guard ID x Minute = ");
}

//fn get_sorted_observation(contents)

fn parse_line(line: &str) -> Observation {
    let line: Vec<_> = line[1..].split(']').collect();
    let message: Vec<_> = line[1].split_whitespace().collect();
    let mut observation = Observation {
        date_time: NaiveDateTime::parse_from_str(&line[0], "%Y-%m-%d %H:%M").unwrap(),
        ..Default::default()
    };
    match message[0] {
        "Guard" => observation.guard_id = message[1][1..].to_string(),
        "falls" => observation.asleep = true,
        "wakes" => observation.awake = true,
        _ => panic!("Unexpected message"),
    }
    observation
}
