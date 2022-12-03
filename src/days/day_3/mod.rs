// Day 3: Rucksack Reorganization
// https://adventofcode.com/2022/day/3

pub fn run() -> (String, String) {
    // Testing
    let alphabet: Vec<u8> = (b'a'..=b'z').chain(b'A'..=b'Z').collect();

    let sum_of_rucksack_items = include_str!("input.txt")
        .lines()
        .map(|rucksack| {
            let characters: Vec<&[u8]> = rucksack
                .as_bytes()
                .chunks(rucksack.as_bytes().len() / 2)
                .collect();

            // Get common element
            let left = characters.get(0).unwrap();
            let right = characters.get(1).unwrap();
            let common = left.iter().find(|item| right.contains(item)).unwrap();

            // Find its value in the normalized alphabet vector
            let item_value = alphabet.iter().position(|letter| letter == common).unwrap() + 1;
            return item_value as u32;
        })
        .sum::<u32>();

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
        }).sum::<u32>();

    return (sum_of_rucksack_items.to_string(), sum_of_group_items.to_string());
}
