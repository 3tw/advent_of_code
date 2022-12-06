// Day 4: Camp Cleanup
// https://adventofcode.com/2022/day/4

pub fn run() -> (String, String) {
    // Part 1
    let fully_overlapping_pairs = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let left = left.split_once('-').unwrap();
            let right = right.split_once('-').unwrap();

            let left = (
                left.0.parse::<u32>().unwrap(),
                left.1.parse::<u32>().unwrap(),
            );
            let right = (
                right.0.parse::<u32>().unwrap(),
                right.1.parse::<u32>().unwrap(),
            );
            return (left, right);
        })
        .filter(|((l_from, l_to), (r_from, r_to))| {
            // Left and right range are overlapping if
            (l_from >= r_from && l_to <= r_to) || (r_from >= l_from && r_to <= l_to)
        })
        .count();

    // Part 2
    let partially_overlapping_pairs = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let left = left.split_once('-').unwrap();
            let right = right.split_once('-').unwrap();

            let left = (
                left.0.parse::<u32>().unwrap(),
                left.1.parse::<u32>().unwrap(),
            );
            let right = (
                right.0.parse::<u32>().unwrap(),
                right.1.parse::<u32>().unwrap(),
            );
            return (left, right);
        })
        .filter(|((l_from, l_to), (r_from, r_to))| {
            // Left and right range are overlapping if
            (l_from >= r_from && l_from <= r_to)
                || (l_to >= r_from && l_to <= r_to)
                || (r_from >= l_from && r_from <= l_to)
                || (r_to >= l_from && r_to <= l_to)
        })
        .count();

    return (
        fully_overlapping_pairs.to_string(),
        partially_overlapping_pairs.to_string(),
    );
}
