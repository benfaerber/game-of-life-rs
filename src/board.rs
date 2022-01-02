use std::fmt;
use std::fs;
use std::process;

#[path = "./cell.rs"]
mod cell;
type Cell = cell::Cell;

#[path = "./point.rs"]
mod point;
type Point = point::Point;

#[path = "./parser.rs"]
mod parser;

type Grid = Vec<Vec<Cell>>;

#[derive(PartialEq, Debug)]
pub struct Board {
  pub grid: Grid
}

impl Board {
  pub fn from(grid: Grid) -> Self {
    Self { grid }
  }

  pub fn from_file(filename: &str) -> Self {

    let err = || "Error!".to_string();
    let text = fs::read_to_string(format!("boards/{}", filename))
    .unwrap_or(err());

    if text == err() {
      println!("The file {} does not exist! Please try again.", filename);
      process::exit(1)
    }

    let ext = parser::get_extenstion(filename);
    match ext.as_str() {
      "txt" => Self::from_txt_file(text),
      "cell" => Self::from_cell_file(text),
      _ => Self::from_txt_file(text)
    }
  }

  fn from_txt_file(text:  String) -> Self {
    let grid = text
    .trim()
    .split('\n')
    .map(|str_y| str_y
      .split("")
      .filter(|l| l.to_owned() != "")
      .map(|l| Cell::from_str(l))
      .collect()
    )
    .collect();

    Self::from(grid)
  }

  fn from_cell_file(text: String) -> Self {
    let meta = parser::get_metadata(text.as_str());
    println!("{:?}", meta);

    let raw_cell_lines = parser::find_significant_lines(text.split("\n").collect());
    let pad_cell_lines = parser::pad_cell_lines(raw_cell_lines);
    let grid: Grid = pad_cell_lines
    .iter()
    .map(|line| {
      line
      .replace("O", "o")
      .split("")
      .map(|l| Cell::from_str(l))
      .collect()
    })
    .collect();

    Self::from(grid)
  }

  fn render(&self) -> String {
    let blank = || "".to_string();
    self.grid
    .iter()
    .fold(blank(), |acc, row| {
      let str_row = row
      .iter()
      .fold(blank(), |acc, current_cell| {
        format!("{}{}", acc, current_cell)
      });

      format!("{}{}\n", acc, str_row)
    })
  }

  pub fn count_neighbors(&self, cell_loc: Point) -> i32 {
    let range = || -1..=1;
    range().fold(0, |neighbors, y_mod| {
      neighbors + range().fold(0, |c_neighbors, x_mod| {
        let no_change = y_mod == 0 && x_mod == 0;
        let offset = cell_loc.add(Point::from(x_mod, y_mod));
        if no_change || !self.in_range(offset) {
          c_neighbors
        } else {
          let alive_offset = self.is_alive(offset) as i32;
          c_neighbors + alive_offset
        }
      })
    })
  }

  pub fn simulate_step(&self) -> Self {
    let grid = self.grid
    .iter()
    .enumerate()
    .map(|(y_index, row)| {
      row
      .iter()
      .enumerate()
      .map(|(x_index, current_cell)| {
        let cell_loc = Point::from_usize(x_index, y_index);
        let neighbors = self.count_neighbors(cell_loc);
        let was_alive = current_cell.to_owned() == Cell::Alive;
        let is_alive = neighbors == 3 || (was_alive && neighbors == 2);

        Cell::alive(is_alive)
      })
      .collect()
    })
    .collect();

    Self::from(grid)
  }

  pub fn get_cell(&self, point: Point) -> Cell {
    self.grid[point.y as usize][point.x as usize]
  }

  pub fn is_alive(&self, point: Point) -> bool {
    self.get_cell(point) == Cell::Alive
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