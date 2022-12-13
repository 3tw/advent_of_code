use super::point::Point;

pub struct Rope {
    pub head: Point,
    pub tail: [Point;9],
}

impl Rope {
    pub fn change(&mut self, direction: &str) {
        let (x, y) = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "D" => (0, -1),
            _ => (0, 1),
        };

        // If tail and head occupy same point, move only head
        let move_head_only: bool = self.head == self.tail;
        self.head.x += x;
        self.head.y += y;
        
        if move_head_only == true {
            return;
        }

        // Move the other nodes
        self.move_node()
    }

    fn move_node(&mut self) {
        let distance = self.head - self.tail;
        if distance.x.abs() > 1 || distance.y.abs() > 1 {
            match distance.x {
                x if x == 2 => self.tail.x += 1,
                x if x == -2 => self.tail.x -= 1,
                _x if distance.y.abs() == 2 => self.tail.x = self.head.x,
                _ => (),
            }
            match distance.y {
                y if y == 2 => self.tail.y += 1,
                y if y == -2 => self.tail.y -= 1,
                _y if distance.x.abs() == 2 => self.tail.y = self.head.y,
                _ => (),
            }
        }
    }
}
