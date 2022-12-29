// Day 12: Hill Climbing Algorithm
// https://adventofcode.com/2022/day/12

mod coord;
mod find;
use coord::Coord;
use find::Find;
use grid::*;

pub fn run() -> (String, String) {
    let input = include_str!("input.txt");
    let mut rows: usize = 0;
    let mut columns: usize = 0;
    let mut parsed = vec![];
    let mut start = Coord::new(0, 0);
    let mut end = Coord::new(0, 0);
    input.as_bytes().into_iter().for_each(|&character| {
        if character == 'S' as u8 {
            start.row = rows;
            start.col = columns;
        }
        if character == 'E' as u8 {
            end.row = rows;
            end.col = columns;
        }
        if character == '\n' as u8 {
            columns = 0;
            rows += 1;
        } else {
            parsed.push(character);
            columns += 1;
        }
    });

    let mut navigate = Find::new(Grid::from_vec(parsed.to_vec(), columns), start, end);
    navigate.find_shortest_path('E', "up");
    let moves = navigate.count;
    let mut navigate_2 = Find::new(Grid::from_vec(parsed, columns), end, start);
    navigate_2.find_shortest_path('a', "down");
    let moves_2 = navigate_2.count;

    return (moves.to_string(), moves_2.to_string());
}
