use std::fs;
use std::env;
use std::{thread, time};

mod board;
mod help;

const DEFAULT_FILENAME: &str = "glider.txt";
const DEFAULT_DAYS: i32 = 25;
const ANIMATION_FRAME_DURATION_MS: u64 = 250;

fn simulate_step(board: &board::Board) -> board::Board {
  let grid = board.grid
  .iter()
  .enumerate()
  .map(|(y_index, row)| {
    row
    .iter()
    .enumerate()
    .map(|(x_index, cell)| {
      let cell_loc = board::Point { x: x_index as i32, y: y_index as i32 };
      let neighbors = board.count_neighbors(cell_loc);
      let was_alive = cell == &board::Cell::Alive;
      let is_alive = if was_alive {
        neighbors == 2 || neighbors == 3
      } else {
        neighbors == 3
      };

      if is_alive { board::Cell::Alive } else { board::Cell::Dead }
    })
    .collect()
  })
  .collect();

  board::Board::from(grid)
}

struct Simulation {
  name: String,
  board: board::Board,
  days: i32,
  delayed: bool
}

impl Simulation {
  fn run(&self) {
    let mut output = "".to_string();

    let start_board = board::Board::from(self.board.grid.clone());

    (0..self.days).fold(start_board, |last_board, day_index| {
      let daily_output = format!("Day {}:\n{}\n", day_index + 1, &last_board);
      print!("{}", daily_output);
      output.push_str(&daily_output);

      if self.delayed {
        let sleep_period = time::Duration::from_millis(ANIMATION_FRAME_DURATION_MS);
        thread::sleep(sleep_period);
      }

      simulate_step(&last_board)
    });

    self.save(output);
  }

  fn save(&self, output: String) {
    let parts: Vec<&str> = self.name.split(".").collect();
    let base: &str = parts[0];

    let filepath = format!("output/{}-{}-days.txt", base, self.days);
    fs::write(filepath, output).expect("Error writing file!");
  }
}


fn load_simulation(filename: &str, days: i32) {
  let board: board::Board = board::Board::from_file(filename);

  let simulation = Simulation {
    name: filename.to_string(),
    board: board,
    days: days,
    delayed: true
  };

  simulation.run();
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  match args.len() {
    1 => {
      help::display(DEFAULT_FILENAME, DEFAULT_DAYS)
    },
    2 => {
      let filename = args[1].as_str();
      load_simulation(filename, DEFAULT_DAYS);
    },
    3 => {
      let filename = args[1].as_str();
      let days = args[2].parse::<i32>().unwrap_or(0);
      load_simulation(filename, days)
    },
    _ => load_simulation(DEFAULT_FILENAME, DEFAULT_DAYS),
  }
}
