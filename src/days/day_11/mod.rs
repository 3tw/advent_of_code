// Day 11: Monkey in the Middle
// https://adventofcode.com/2022/day/11

mod monkey;
use itertools::Itertools;
use monkey::Monkey;
use std::collections::VecDeque;

pub fn run() -> (String, String) {
    let input = include_str!("input.txt").split("\n\n");
    let mut common_denominator = 1;
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkeys_2: Vec<Monkey> = vec![];

    input.for_each(|monkey| {
        let info: Vec<&str> = monkey.lines().collect();
        // Get items
        let skip = "Starting items:".to_string().len();
        let items_raw = info[1].trim().to_string();
        let items_raw = &items_raw[skip..];
        let items: VecDeque<u64> = items_raw
            .split(",")
            .map(|item| item.trim().parse::<u64>().unwrap())
            .collect();

        // Get inspect operation
        let inspect_arguments = info[2]
            .split_once("= ")
            .unwrap()
            .1
            .split(" ")
            .skip(1)
            .take(2)
            .map(|s| s.to_string())
            .collect_tuple()
            .unwrap();

        // Get test divisor
        let divisor = info[3].split_once("by ").unwrap().1.parse::<u64>().unwrap();
        common_denominator *= divisor;

        // Get target
        let target = [info[4], info[5]];
        let target: (u8, u8) = target
            .into_iter()
            .map(|raw| raw.split_once("monkey ").unwrap().1.parse::<u8>().unwrap())
            .collect_tuple()
            .unwrap();

        let monkey = Monkey::new(items, inspect_arguments, divisor, target);
        let monkey_2 = monkey.clone();
        monkeys.push(monkey);
        monkeys_2.push(monkey_2);
    });

    // Part 1
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            if monkeys[i].items.len() == 0 {
                continue;
            }
            let move_queue = monkeys[i].process("divide", 3);
            move_queue
                .into_iter()
                .for_each(|instruction| monkeys[instruction.0 as usize].push(instruction.1))
        }
    }
    let part_1 = get_product_of_top_two(monkeys);

    // Part 2
    for _ in 0..10000 {
        for i in 0..monkeys_2.len() {
            if monkeys_2[i].items.len() == 0 {
                continue;
            }
            let move_queue = monkeys_2[i].process("modulo", common_denominator);
            move_queue
                .into_iter()
                .for_each(|instruction| monkeys_2[instruction.0 as usize].push(instruction.1))
        }
    }
   
    let part_2 = get_product_of_top_two(monkeys_2);

    return (part_1.to_string(), part_2.to_string());
}

fn get_product_of_top_two(monkeys: Vec<Monkey>) -> u64 {
    monkeys
        .into_iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}
