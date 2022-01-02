#![allow(dead_code)]

use std::env;

mod help;
mod simulation;
type Simulation = simulation::Simulation;

const DEFAULT_FILENAME: &str = "glider.txt";
const DEFAULT_DAYS: i32 = 25;

fn cli() {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => help::display(DEFAULT_FILENAME, DEFAULT_DAYS),
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

fn test() {


  Simulation::load("example.cell", DEFAULT_DAYS);
}

fn main() {
  cli()
}
