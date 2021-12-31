use std::fs;
use std::env;
use std::{thread, time};

mod board;

const DEFAULT_FILENAME: &str = "glider.txt";
const DEFAULT_DAYS: i32 = 25;
const ANIMATION_FRAME_DURATION_MS: u64 = 250;

fn save_simulation_output(inputname: &str, steps: i32, output: String) {
  let parts: Vec<&str> = inputname.split(".").collect();
  let base: &str = parts[0];

  let filepath = format!("output/{}-{}-days.txt", base, steps);
  fs::write(filepath, output).expect("Error writing file!");
}

fn count_neighbors(board: &board::Board, cell_loc: board::Point) -> i32 {
  let range_y = -1..=1;
  range_y.fold(0, |neighbors, y_mod| {
    let range_x = -1..=1;

    neighbors + range_x.fold(0, |c_neighbors, x_mod| {
      let offset = cell_loc.add(board::Point {x: x_mod, y: y_mod});
      let no_change = y_mod == 0 && x_mod == 0;
      if no_change || !board.in_range(&offset) { return c_neighbors }

      let alive_offset = board.is_alive(offset) as i32;
      c_neighbors + alive_offset
    })
  })
}

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
      let neighbors = count_neighbors(&board, cell_loc);
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

fn run_simulation(simulation: Simulation) {
  let Simulation {name, board, days, delayed} = simulation;

  let mut output = "".to_string();

  (0..days).fold(board, |last_board, day_index| {
    let daily_output = format!("Day {}:\n{}\n", day_index + 1, &last_board);
    print!("{}", daily_output);
    output.push_str(&daily_output);

    if delayed {
      let sleep_period = time::Duration::from_millis(ANIMATION_FRAME_DURATION_MS);
      thread::sleep(sleep_period);
    }

    simulate_step(&last_board)
  });

  save_simulation_output(name.as_str(), days, output);
}

fn load_simulation(filename: &str, days: i32) {
  let board: board::Board = board::Board::from_file(filename);

  let sim = Simulation {
    name: filename.to_string(),
    board: board,
    days: days,
    delayed: true
  };

  run_simulation(sim);
}

fn display_help() {
  let title = "Conway's Game of Life Rust";
  let underline = str::repeat("-", title.len());
  println!("{}\n{}", title, underline);


  let mut param_count = 1;
  let mut print_param = |p: &str, def: &str| {
    println!("  {}. {}", param_count, p);
    println!("     Default: {}", def);
    param_count += 1;
  };

  println!("Params:");
  print_param("Filename: the board to start the simulation with, pulled from boards folder, must end in .txt", DEFAULT_FILENAME);
  print_param("Days: the number of days (iterations) to run the simulation", DEFAULT_DAYS.to_string().as_str());

  println!("\nExamples:");
  println!("  game-of-life-rs glider.txt");
  println!("  game-of-life-rs calculator.txt 200");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  match args.len() {
    1 => {
      display_help()
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
