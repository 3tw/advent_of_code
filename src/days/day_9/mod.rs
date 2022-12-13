// Day 9: Rope Bridge
// https://adventofcode.com/2022/day/9

mod movement;
mod point;

use movement::Rope;
use point::Point;
use std::collections::HashSet;

pub fn run() -> (String, String) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut visited_short_tail: HashSet<String> = HashSet::new();
    let mut visited_long_tail: HashSet<String> = HashSet::new();
    
    // Current positions
    let default = Point { x: 0, y: 0 };
    let mut rope = Rope {
        nodes: [default; 10],
    };

    // Calculate movements
    for line in input {
        let (direction, step) = line.split_once(" ").unwrap();

        // For each moving step get tail position and push it to hashset
        for _ in 0..step.parse::<i32>().unwrap() {
            let (first_node, tail) = rope.change(direction);
            visited_short_tail.insert(first_node);
            visited_long_tail.insert(tail);
        }
    }

    return (visited_short_tail.len().to_string(), visited_long_tail.len().to_string());
}
