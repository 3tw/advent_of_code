#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        return Self { row, col };
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}