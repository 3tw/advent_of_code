// Day 1: Calorie Counting
// https://adventofcode.com/2022/day/1

use crate::constants::RUN;
use crate::utils;

pub fn run() -> (String, String) {
    let mut list_of_sums: Vec<u32> = Vec::new();
    let mut current_sum = 0;
    println!("\nPlease enter provided input for the puzzle:");

    loop {
        let input = utils::get_input();
        match input.trim().parse::<u32>() {
            Ok(number) => current_sum += number,
            Err(_) => {
                list_of_sums.push(current_sum);
                current_sum = 0
            }
        }
        if input == RUN {
            break;
        }
    }

    list_of_sums.sort_by(|a, b| b.cmp(a));
    let sum_of_top_three: u32 = list_of_sums[0 .. 3].iter().sum();

    return (list_of_sums[0].to_string(), sum_of_top_three.to_string());
}
