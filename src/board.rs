use std::fmt;
use std::fs;

#[path = "./cell.rs"]
mod cell;

#[derive(PartialEq, Debug, Clone, Copy)]
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
  pub grid: Vec<Vec<cell::Cell>>
}

impl Board {
  pub fn from(grid: Vec<Vec<cell::Cell>>) -> Board {
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
      .map(|l| cell::Cell::from_str(l))
      .collect()
    )
    .collect();

    Board::from(grid)
  }

  pub fn render(&self) -> String {
    let blank = || "".to_string();
    self.grid
    .iter()
    .fold(blank(), |acc, row| {
      let str_row = row
      .iter()
      .fold(blank(), |acc, cell| {
        format!("{}{}", acc, cell)
      });

      format!("{}{}\n", acc, str_row)
    })
  }

  pub fn count_neighbors(&self, cell_loc: Point) -> i32 {
    let range = || -1..=1;
    range().fold(0, |neighbors, y_mod| {
      neighbors + range().fold(0, |c_neighbors, x_mod| {
        let no_change = y_mod == 0 && x_mod == 0;
        let offset = cell_loc.add(Point {x: x_mod, y: y_mod});
        if no_change || !self.in_range(offset) {
          c_neighbors
        } else {
          let alive_offset = self.is_alive(offset) as i32;
          c_neighbors + alive_offset
        }
      })
    })
  }

  pub fn simulate_step(&self) -> Board {
    let grid = self.grid
    .iter()
    .enumerate()
    .map(|(y_index, row)| {
      row
      .iter()
      .enumerate()
      .map(|(x_index, current_cell)| {
        let cell_loc = Point { x: x_index as i32, y: y_index as i32 };
        let neighbors = self.count_neighbors(cell_loc);
        let was_alive = current_cell == &cell::Cell::Alive;
        let is_alive = if was_alive {
          neighbors == 2 || neighbors == 3
        } else {
          neighbors == 3
        };

        if is_alive { cell::Cell::Alive } else { cell::Cell::Dead }
      })
      .collect()
    })
    .collect();

    Board::from(grid)
  }

  pub fn get_cell(&self, point: Point) -> cell::Cell {
    self.grid[point.y as usize][point.x as usize]
  }

  pub fn is_alive(&self, point: Point) -> bool {
    self.get_cell(point) == cell::Cell::Alive
  }

  pub fn in_range(&self, point: Point) -> bool {
    let Point {x, y} = point;
    let lower_limit = y < 0 || x < 0;
    let g = &self.grid;
    let upper_limit = y >= (g.len() as i32) || x >= (g[0].len() as i32);
    !lower_limit && !upper_limit
  }

}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.render())
  }
}