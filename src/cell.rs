use std::fmt;

const GUI_CELL: &str = "(_)";
const GUI_BLANK: &str = " . ";
const FILE_CELL: &str = "o";
const FILE_BLANK: &str = ".";

#[derive(PartialEq, Debug, Clone, Copy)]
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

impl fmt::Display for Cell {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_string())
  }
}