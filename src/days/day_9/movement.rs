use super::point::Point;

pub struct Rope {
    pub nodes: [Point; 10],
}

impl Rope {
    pub fn change(&mut self, direction: &str) -> (String, String) {
        let (x, y) = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            _ => (0, 1),
        };

        // Move head
        let stop_current_node: bool = self.nodes[0] == self.nodes[1];
        self.nodes[0].x += x;
        self.nodes[0].y += y;

        // If head and second node occupy same point exit early
        if stop_current_node == true {
            return self.get_tail_position();
        }

        // Loop through nodes and move each one if the preceding is more than 2 units away
        for index in 1..self.nodes.len() {
            let distance = self.nodes[index - 1] - self.nodes[index];
            if distance.x.abs() < 1 && distance.y.abs() < 1 {
                break;
            }
            self.move_node(index, distance);
        }

        return self.get_tail_position();
    }

    fn move_node(&mut self, index: usize, distance: Point) {
        match distance.x {
            x if x == 2 => self.nodes[index].x += 1,
            x if x == -2 => self.nodes[index].x -= 1,
            _x if distance.y.abs() == 2 => self.nodes[index].x = self.nodes[index - 1].x,
            _ => (),
        }
        match distance.y {
            y if y == 2 => self.nodes[index].y += 1,
            y if y == -2 => self.nodes[index].y -= 1,
            _y if distance.x.abs() == 2 => self.nodes[index].y = self.nodes[index - 1].y,
            _ => (),
        }
    }

    fn get_tail_position(&self) -> (String, String) {
        let first_node = self.nodes[1].x.to_string() + &self.nodes[1].y.to_string();
        let tail = self.nodes[self.nodes.len() - 1].x.to_string() + &self.nodes[self.nodes.len() - 1].y.to_string();
        return (first_node, tail);
    }
}
