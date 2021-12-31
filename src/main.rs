use std::fs;
use std::env;
use std::{thread, time};

mod board;
mod help;

const DEFAULT_FILENAME: &str = "glider.txt";
const DEFAULT_DAYS: i32 = 25;
const ANIMATION_FRAME_DURATION_MS: u64 = 250;

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

      last_board.simulate_step()
    });

    self.save(output);
  }

  fn save(&self, output: String) {
    let parts: Vec<&str> = self.name.split(".").collect();
    let base: &str = parts[0];

    let filepath = format!("output/{}-{}-days.txt", base, self.days);
    fs::write(filepath, output).expect("Error writing file!");
  }

  fn load(filename: &str, days: i32) -> Simulation {
    let board: board::Board = board::Board::from_file(filename);

    Simulation {
      name: filename.to_string(),
      board: board,
      days: days,
      delayed: true
    }
  }
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
      Simulation::load(filename, DEFAULT_DAYS).run();
    },
    3 => {
      let filename = args[1].as_str();
      let days = args[2].parse::<i32>().unwrap_or(0);
      Simulation::load(filename, days).run()
    },
    _ => Simulation::load(DEFAULT_FILENAME, DEFAULT_DAYS).run(),
  }
}
