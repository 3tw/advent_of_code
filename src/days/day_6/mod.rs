// Day 6: Tuning Trouble
// https://adventofcode.com/2022/day/6

use itertools::Itertools;

pub fn run() -> (String, String) {
    let input: Vec<u8> = include_str!("input.txt")
        .as_bytes()
        .into_iter()
        .map(|&i| i)
        .collect();

    // Part 1
    let mut i: usize = 0;
    loop {
        let chunk = &input[i..(i + 4)]
            .into_iter()
            .map(|&i| i)
            .unique()
            .collect::<Vec<u8>>();
        if chunk.len() == 4 {
            break;
        }
        i += 1;
    }
    let last_char_index_1 = i + 4;

    // Part 2
    let mut j: usize = 0;
    loop {
        let chunk = &input[j..(j + 14)]
            .into_iter()
            .map(|&j| j)
            .unique()
            .collect::<Vec<u8>>();
        if chunk.len() == 14 {
            break;
        }
        j += 1;
    }
    let last_char_index_2 = j + 14;

    return (last_char_index_1.to_string(), last_char_index_2.to_string());
}
