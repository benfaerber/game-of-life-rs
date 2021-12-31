use std::fmt;
use std::fs;

const GUI_CELL: &str = "(_)";
const GUI_BLANK: &str = " . ";
const FILE_CELL: &str = "o";
const FILE_BLANK: &str = ".";

#[derive(PartialEq, Debug)]
pub enum Cell {
  Dead,
  Alive
}

impl Cell {
  pub fn to_string(&self) -> String {
    match self {
      Cell::Alive => GUI_CELL,
      Cell::Dead => GUI_BLANK
    }.to_string()
  }

  pub fn from_str(s: &str) -> Cell {
    match s {
      FILE_CELL => Cell::Alive,
      FILE_BLANK => Cell::Dead,
      _ => Cell::Dead
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl Point {
  pub fn add(&self, other: Point) -> Point {
    Point { x: self.x + other.x, y: self.y + other.y }
  }
}

#[derive(PartialEq, Debug)]
pub struct Board {
  pub grid: Vec<Vec<Cell>>
}

impl Board {
  pub fn from(grid: Vec<Vec<Cell>>) -> Board {
    Board { grid }
  }

  pub fn from_file(filename: &str) -> Board {
    let text = fs::read_to_string(format!("boards/{}", filename))
    .expect("There was an error reading this file!");

    let grid = text
    .trim()
    .split('\n')
    .map(|str_y| str_y
      .split("")
      .filter(|l| l != &"")
      .map(|l| Cell::from_str(l))
      .collect()
    )
    .collect();

    Board::from(grid)
  }

  pub fn render(&self) -> String {
    self.grid
    .iter()
    .fold("".to_string(), |acc, row| {
      let str_row = row
      .iter()
      .fold("".to_string(), |acc, cell| {
        format!("{}{}", acc, cell.to_string())
      });

      format!("{}{}\n", acc, str_row)
    })
  }

  pub fn get_cell(&self, point: Point) -> Cell {
    let result = &self.grid[point.y as usize][point.x as usize];
    // Copy enum
    if result == &Cell::Alive { Cell::Alive } else { Cell::Dead }
  }

  pub fn is_alive(&self, point: Point) -> bool {
    self.get_cell(point) == Cell::Alive
  }

  pub fn in_range(&self, point: &Point) -> bool {
    let lower_limit = point.y < 0 || point.x < 0;
    let upper_limit = point.y >= (self.grid.len() as i32) || point.x >= (self.grid[0].len() as i32);
    !lower_limit && !upper_limit
  }

}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.render())
  }
}