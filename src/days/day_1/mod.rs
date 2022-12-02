// Day 1: Calorie Counting
// https://adventofcode.com/2022/day/1

pub fn run() -> (String, String) {
    let mut all_sums = include_str!("input.txt")
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    all_sums.sort();
    
    let max_value = all_sums[all_sums.len() -1].to_string();
    let sum_of_top_three = all_sums.into_iter().rev().take(3).sum::<u32>().to_string();

    return (max_value, sum_of_top_three);
}
