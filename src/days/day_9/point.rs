use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Sub for Point {
  type Output = Self;
  
  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}