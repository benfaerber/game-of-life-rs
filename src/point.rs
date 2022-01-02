use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl Point {
  pub fn from(x: i32, y: i32) -> Self {
    Self {x, y}
  }

  pub fn from_usize(x: usize, y: usize) -> Self {
    Self { x: x as i32, y: y as i32 }
  }

  pub fn add(&self, other: Point) -> Self {
    Self { x: self.x + other.x, y: self.y + other.y }
  }

  pub fn eq(&self, other: Point) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format!("{}, {}", self.x, self.y))
  }
}