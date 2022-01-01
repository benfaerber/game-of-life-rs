use std::fmt;

enum DisplayType {
  Emoji,
  Ascii
}

const DISPLAY_TYPE: DisplayType = DisplayType::Ascii;

const ASCII_CELL: &str = "(_)";
const ASCII_BLANK: &str = " . ";
const EMOJI_CELL: &str = "🦠";
const EMOJI_BLANK: &str = "⚫";

const FILE_CELL: &str = "o";
const FILE_BLANK: &str = ".";

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Cell {
  Dead,
  Alive
}

impl Cell {
  pub fn alive(is_alive: bool) -> Cell {
    if is_alive {
      Cell::Alive
    } else {
      Cell::Dead
    }
  }

  pub fn to_string(&self) -> String {
    match (self, DISPLAY_TYPE) {
      (Cell::Alive, DisplayType::Ascii) => ASCII_CELL,
      (Cell::Dead,  DisplayType::Ascii) => ASCII_BLANK,
      (Cell::Alive, DisplayType::Emoji) => EMOJI_CELL,
      (Cell::Dead,  DisplayType::Emoji) => EMOJI_BLANK,
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