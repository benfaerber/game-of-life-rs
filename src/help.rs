
pub fn display(default_filename: &str, default_days: i32) {
  let title = "Conway's Game of Life Rust";
  let underline = str::repeat("-", title.len());
  println!("{}\n{}", title, underline);


  let tab = |indent: usize| str::repeat("  ", indent);
  let mut param_count = 1;
  let mut print_param = |p: &str, def: &str| {
    println!("{}{}. {}", tab(1), param_count, p);
    println!("{} Default: {}", tab(2), def);
    param_count += 1;
  };

  println!("Params:");
  print_param("Filename: the board to start the simulation with, pulled from boards folder, must end in .txt", default_filename);
  print_param("Days: the number of days (iterations) to run the simulation", default_days.to_string().as_str());

  println!("\nExamples:");
  println!("  game-of-life-rs glider.txt");
  println!("  game-of-life-rs calculator.txt 200");
}