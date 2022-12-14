// Day 10: Cathode-Ray Tube
// https://adventofcode.com/2022/day/10

mod utils;
use utils::{calculate_signal_strength, draw_pixel};

pub fn run() -> (String, String) {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let mut cycle: usize = 1;
    let mut register: i32 = 1;
    let mut signal_strength = 0;
    let mut crt_row: Vec<String> = vec![];

    for line in input {
        // Get current value
        signal_strength += calculate_signal_strength(cycle, register);
        /*
         * Push pixel to CRT
         * Cycle 1 starts drawing at pixel 0,
         */
        crt_row.push(draw_pixel((cycle as i32 - 1) % 40, register));
        if line == "noop" {
            cycle += 1;
            continue;
        }

        // Process instruction
        let (_, value) = line.split_once(" ").unwrap();
        cycle += 1;
        crt_row.push(draw_pixel((cycle as i32 - 1) % 40, register));
        signal_strength += calculate_signal_strength(cycle, register);
        register += value.parse::<i32>().unwrap();
        cycle += 1;
    }

    // Part 2 solution
    for chunk in crt_row.chunks(40) {
        for c in chunk {
            print!("{c}");
        }
        print!("\n");
    }
    return (
        signal_strength.to_string(),
        "Solution of part 2 is above".to_string(),
    );
}
