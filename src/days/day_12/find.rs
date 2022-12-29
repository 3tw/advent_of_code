use super::coord::Coord;
use grid::*;
use std::collections::VecDeque;

pub struct Find {
    pub elevation_map: Grid<u8>,
    pub start: Coord,
    pub end: Coord,
    pub count: usize,
}

impl Find {
    pub fn new(grid: Grid<u8>, start: Coord, end: Coord) -> Self {
        return Self {
            elevation_map: grid,
            start,
            end,
            count: 1,
        };
    }

    /*
     * Implement Breadth First Search
     * 1. Create grid, that mirrors the exiting one to mark visited items with 1
     * 2. Create an empty queue
     * 3. Keep track of nodes added in each step, to count correctly
     */
    pub fn find_shortest_path(&mut self, end_char: char, direction: &str) {
        let end = end_char as u8;
        let mut visited: Grid<u8> = Grid::new(self.elevation_map.rows(), self.elevation_map.cols());
        let mut queue: VecDeque<Coord> = VecDeque::new();
        queue.push_front(self.start);
        let current_visited = visited.get_mut(self.start.row, self.start.col).unwrap();
        *current_visited = 1;

        let mut nodes_in_layer = 1;
        let mut nodes_in_next_layer = 0;

        let mut last_step = 'z' as u8;
        if direction == "down" {
            last_step = 'b' as u8;
        };

        'while_loop: while queue.len() != 0 {
            let current = queue.pop_front().unwrap();
            let adjacent_nodes = self.get_adjacent_nodes(current);

            for node in adjacent_nodes.iter() {
                let item = match self.elevation_map.get(node.row, node.col) {
                    Some(item) => *item,
                    None => continue,
                };

                // In part 1 only z can jump to E (instructions somehow unclear);
                // and only b can jump to a in part 1
                if item == end
                    && *self.elevation_map.get(current.row, current.col).unwrap() == last_step
                {
                    break 'while_loop;
                }

                let current_visited = visited.get_mut(node.row, node.col).unwrap();
                if *current_visited != 1 && self.item_fits(current, *node, direction) {
                    queue.push_back(*node);
                    nodes_in_next_layer += 1;
                    *current_visited = 1;
                }
            }
            nodes_in_layer -= 1;
            if nodes_in_layer == 0 {
                nodes_in_layer = nodes_in_next_layer;
                nodes_in_next_layer = 0;
                self.count += 1;
            }
        }
    }

    fn item_fits(&self, current: Coord, next: Coord, direction: &str) -> bool {
        let current = *self.elevation_map.get(current.row, current.col).unwrap();
        let next = *self.elevation_map.get(next.row, next.col).unwrap();
        match direction {
            "up" => {
                if current == 83 || current >= next - 1 {
                    return true;
                }
                return false;
            }
            _ => {
                if current == 69 && next == 'z' as u8 {
                    return true;
                } else if current != 69 && current - 1 <= next {
                    return true;
                }
                return false;
            }
        };
    }

    fn get_adjacent_nodes(&self, current: Coord) -> Vec<Coord> {
        let col = current.col as i32;
        let row = current.row as i32;
        let nodes = vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ];

        return nodes
            .into_iter()
            .filter(|(a, b)| *a >= 0 && *b >= 0)
            .map(|(a, b)| Coord::new(a as usize, b as usize))
            .collect::<Vec<Coord>>();
    }
}
