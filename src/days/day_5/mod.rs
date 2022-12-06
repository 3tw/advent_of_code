// Day 5: Supply Stacks
// https://adventofcode.com/2022/day/5
mod parse;
use crate::days::day_5::parse::{parse_columns, parse_instructions};

pub fn run() -> (String, String) {
    // Parse input
    let (columns, instructions) = include_str!("input.txt").split_once("\n\n").unwrap();
    let columns = columns.lines().collect::<Vec<&str>>();

    let instructions: Vec<&str> = instructions.lines().collect();
    let instructions = parse_instructions(instructions);

    pub fn get_top_crates(columns: &mut Vec<Vec<char>>) -> String {
        let last_items: String = columns.into_iter().map(|col| col.pop().unwrap()).collect();
        return last_items;
    }

    // Part 1
    let mut columns_1 = parse_columns(columns.to_vec());
    for command in &instructions {
        let amount = command[0];
        for _ in 0..amount {
            let from = command[1] - 1;
            let to = command[2] - 1;
            let moved = columns_1[from].pop().unwrap();
            columns_1[to].push(moved);
        }
    }
    let column_tops_1: String = get_top_crates(&mut columns_1);

    // Part 2
    let mut columns_2 = parse_columns(columns.to_vec());
    for command in &instructions {
        let amount = command[0];
        let from = command[1] - 1;
        let to = command[2] - 1;
        println!("{:?}", columns_2[from]);
        let range = columns_2[from].len() - amount;
        let mut moved: Vec<char> = columns_2[from].drain(range..).collect();
        columns_2[to].append(&mut moved);
    }
    let column_tops_2: String = get_top_crates(&mut columns_2);

    return (column_tops_1.to_string(), column_tops_2.to_string());
}
