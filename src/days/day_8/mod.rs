// Day 7: No Space Left On Device
// https://adventofcode.com/2022/day/7

mod file_system;
mod utils;

use crate::days::day_7::file_system::Dir;
use crate::days::day_7::utils::{parse_command, parse_content};
use std::collections::HashMap;

pub fn run() -> (String, String) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    // History
    let mut history: Vec<String> = Vec::new();

    // Setup filesystem
    let mut file_system = HashMap::new();
    file_system.insert("/".to_string(), Dir::new(String::from("/")));
    history.push("".to_string());

    // Initialize loop
    let mut loop_index = 0;
    while loop_index < input.len() {
        let line = input[loop_index];

        if let Some(cmd) = parse_command(line) {
            let (action, target) = cmd;

            // Change directory
            if action == "cd" {
                if target == "/" {
                    // pwd = 0;
                    history.clear();
                    history.push("".to_string());
                } else if target == ".." {
                    history.pop();
                } else {
                    history.push(target.to_string())
                }
                loop_index += 1;
                continue;
            }

            // List content
            if action == "ls" {
                // Get contents
                loop {
                    loop_index += 1;
                    if loop_index == input.len() {
                        break;
                    }
                    if let Some(_) = parse_command(input[loop_index]) {
                        break;
                    }
                    let (a, b) = parse_content(input[loop_index], 0);
                    match a {
                        "dir" => {
                            let name = history.join("/") + "/" + b;
                            file_system.insert(name.to_string(), Dir::new(String::from(b)));
                        }
                        _ => {
                            let key = history.join("/");
                            if let Some(item) = file_system.get_mut(&key) {
                                item.files.push((String::from(b), String::from(a)));
                                item.size += a.parse::<usize>().unwrap();
                            }
                            if let Some(item) = file_system.get_mut("/") {
                                item.files.push((String::from(b), String::from(a)));
                                item.size += a.parse::<usize>().unwrap();
                            }

                            // Update parent directories
                            for i in 1..history.len() {
                                let key = history[0..i].join("/");
                                if let Some(item) = file_system.get_mut(&key) {
                                    item.size += a.parse::<usize>().unwrap();
                                }
                            }
                        }
                    }
                }
                continue;
            }
        } else {
            panic!("The line is not a command")
        }
    }

    // Part 1
    let size_part_1 = file_system
        .values()
        .filter(|&dir| dir.size < 100_000 as usize)
        .map(|i| i.size)
        .sum::<usize>();

    // Part 2
    let free_space = 70_000_000 - file_system.get("/").unwrap().size;
    let size_part_2 = file_system
        .values()
        .filter(|dir| free_space + dir.size > 30_000_000)
        .map(|i| i.size)
        .min()
        .unwrap();

    return (size_part_1.to_string(), size_part_2.to_string());
}
