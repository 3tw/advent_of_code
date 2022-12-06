// Day 3: Rucksack Reorganization
// https://adventofcode.com/2022/day/3

pub fn run() -> (String, String) {
    let alphabet: Vec<u8> = (b'a'..=b'z').chain(b'A'..=b'Z').collect();

    // Part 1
    let sum_of_rucksack_items = include_str!("input.txt")
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            let common = left
                .as_bytes()
                .iter()
                .find(|item| right.as_bytes().contains(item))
                .unwrap();
            let item_value = alphabet.iter().position(|letter| letter == common).unwrap() + 1;
            return item_value as u32;
        })
        .sum::<u32>();

    // Part 2
    let sum_of_group_items = include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|set| {
            set[0]
                .as_bytes()
                .iter()
                .find(|b| set[1].as_bytes().contains(b) && set[2].as_bytes().contains(b))
                .unwrap()
        })
        .map(|common| {
            let item_value = alphabet.iter().position(|letter| letter == common).unwrap() + 1;
            return item_value as u32;
        })
        .sum::<u32>();

    return (
        sum_of_rucksack_items.to_string(),
        sum_of_group_items.to_string(),
    );
}
