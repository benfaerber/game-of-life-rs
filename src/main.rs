use std::fs;
use std::{thread, time};

#[derive(PartialEq, Debug)]
enum Cell {
    Dead,
    Alive,
    Blank
}

impl Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Alive => "(_)",
            Cell::Dead => "...",
            _ => ""
        }.to_string()
    }

    fn from_str(s: &str) -> Cell {
        match s {
            "(_)" => Cell::Alive,
            "..." => Cell::Dead,
            _ => Cell::Blank
        }
    }
}

struct Point {
    x: i32,
    y: i32
}

struct Board {
    grid: Vec<Vec<Cell>>
}

fn render_board(board: &Board) -> String {
    board.grid
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

impl Board {
    fn from(grid: Vec<Vec<Cell>>) -> Board {
        Board { grid }
    }

    fn from_file(filename: &str) -> Board {
        let text = fs::read_to_string(format!("boards/{}", filename))
        .expect("There was an error reading this file!");

        let grid = text
        .trim()
        .split('\n')
        .map(|str_y| str_y
            .trim()
            .split("")
            .map(|letter| Cell::from_str(letter))
            .filter(|c| c != &Cell::Blank)
            .collect()
        )
        .collect();

        Board::from(grid)
    }

    fn render(&self) -> String {
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

    fn get_cell(&self, point: Point) -> Cell {
        let result = &self.grid[point.y as usize][point.x as usize];
        // Copy enum
        if result == &Cell::Alive { Cell::Alive } else { Cell::Dead }
    }

    fn is_alive(&self, point: Point) -> bool {
        self.get_cell(point) == Cell::Alive
    }

}

fn save_simulation_output(inputname: &str, steps: i32, output: String) {
    let parts: Vec<&str> = inputname.split(".").collect();
    let base: &str = parts[0];

    let filepath = format!("output/{}-{}-days.txt", base, steps);
    fs::write(filepath, output).expect("Error writing file!");
}

fn count_neighbors(board: &Board, cell_loc: (i32, i32)) -> i32 {
    let range_y = -1..=1;
    range_y.fold(0, |neighbors, y_mod| {
        let range_x = -1..=1;

        neighbors + range_x.fold(0, |c_neighbors, x_mod| {
            let y = cell_loc.1 + y_mod;
            let x = cell_loc.0 + x_mod;

            let lower_limit = y < 0 || x < 0;
            let upper_limit = y >= (board.grid.len() as i32) || x >= (board.grid[0].len() as i32);
            let no_change = y_mod == 0 && x_mod == 0;
            if lower_limit || upper_limit || no_change { return c_neighbors }

            let is_alive = board.grid[y as usize][x as usize] == Cell::Alive;
            c_neighbors + (is_alive as i32)
        })
    })
}

fn simulate_step(board: &Board) -> Board {
    let grid = board.grid
    .iter()
    .enumerate()
    .map(|(y_index, row)| {
        row
        .iter()
        .enumerate()
        .map(|(x_index, cell)| {
            let neighbors = count_neighbors(&board, (x_index as i32, y_index as i32));
            let was_alive = cell == &Cell::Alive;
            let is_alive = if was_alive {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };

            if is_alive { Cell::Alive } else { Cell::Dead }
        })
        .collect()
    })
    .collect();

    Board::from(grid)
}

fn main() {
    let start_file = "glider.txt";
    let has_delay = true;

    let board: Board = Board::from_file(start_file);
    let mut output = "".to_string();

    let days = 20;
    (0..days).fold(board, |last_board, day_index| {
        let daily_output = format!("Day {}:\n{}\n", day_index + 1, &last_board.render());
        print!("{}", daily_output);
        output.push_str(&daily_output);

        if has_delay {
            let sleep_period = time::Duration::from_millis(100);
            thread::sleep(sleep_period);
        }

        simulate_step(&last_board)
    });

    save_simulation_output(start_file, days, output);
}
