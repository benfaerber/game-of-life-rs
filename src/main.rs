use std::env;

mod board;
mod help;
mod simulation;

const DEFAULT_FILENAME: &str = "glider.txt";
const DEFAULT_DAYS: i32 = 25;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  match args.len() {
    1 => {
      help::display(DEFAULT_FILENAME, DEFAULT_DAYS)
    },
    2 => {
      let filename = args[1].as_str();
      simulation::Simulation::load(filename, DEFAULT_DAYS).run();
    },
    3 => {
      let filename = args[1].as_str();
      let days = args[2].parse::<i32>().unwrap_or(0);
      simulation::Simulation::load(filename, days).run()
    },
    _ => simulation::Simulation::load(DEFAULT_FILENAME, DEFAULT_DAYS).run(),
  }
}
