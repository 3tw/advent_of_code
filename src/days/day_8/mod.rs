// Day 8: Treetop Tree House
// https://adventofcode.com/2022/day/8

mod utils;
use crate::days::day_8::utils::{first_or_last};
use grid::*;
use std::collections::HashMap;

pub fn run() -> (String, String) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut grid: Grid<u8> = Grid::new(0, 0);
    for line in input {
        grid.push_row(
            line.as_bytes()
                .into_iter()
                .map(|&byte| (byte as char).to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );
    }
    let (rows, columns) = grid.size();
    let mut visible_trees = rows * 2 + columns * 2 - 4;
    let mut peaks: HashMap<String, u8> = HashMap::new();

    for row_number in 1..rows - 1 {
        // ltr
        let row_indexed = grid
            .iter_row(row_number)
            .enumerate()
            .collect::<Vec<(usize, &u8)>>();

        let mut current_max: u8 = *row_indexed[0].1;
        for (i, v) in &row_indexed {
            if **v > current_max && !first_or_last(*i, columns) {
                current_max = **v;
                peaks.insert(row_number.to_string() + &i.to_string(), **v);
            }
        }
        // rtl
        let row_indexed = grid
            .iter_row(row_number)
            .enumerate()
            .rev()
            .collect::<Vec<(usize, &u8)>>();

        let mut current_max: u8 = *row_indexed[0].1;
        for (i, v) in &row_indexed {
            if **v > current_max && !first_or_last(*i, columns) {
                current_max = **v;
                peaks.insert(row_number.to_string() + &i.to_string(), **v);
            }
        }
    }

    for column_number in 1..columns - 1 {
        // ttb
        let column_indexed = grid
            .iter_col(column_number)
            .enumerate()
            .collect::<Vec<(usize, &u8)>>();

        let mut current_max: u8 = *column_indexed[0].1;
        for (i, v) in &column_indexed {
            if **v > current_max && !first_or_last(*i, rows) {
                current_max = **v;
                peaks.insert(i.to_string() + &column_number.to_string(), **v);
            }
        }

        // btt
        let column_indexed = grid
            .iter_col(column_number)
            .enumerate()
            .rev()
            .collect::<Vec<(usize, &u8)>>();
        println!("{:?}", column_indexed);

        let mut current_max: u8 = *column_indexed[0].1;
        for (i, v) in &column_indexed {
            if **v > current_max && !first_or_last(*i, rows) {
                current_max = **v;
                peaks.insert(i.to_string() + &column_number.to_string(), **v);
            }
        }
    }

    println!("Side {:?}", visible_trees);
    println!("Visible {:?}", peaks.len());
    visible_trees += peaks.len();
    println!("{:?}", peaks);

    return (visible_trees.to_string(), "size_part_2".to_string());
}
