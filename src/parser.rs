use regex::Regex;
use std::collections::HashMap;

pub fn get_extenstion(filename: &str) -> String {
  let chunks: Vec<&str> = filename.split(".").collect();
  chunks[chunks.len() - 1].to_string()
}

pub fn is_url(potential_url: &str) -> bool {
  let url_regex = Regex::new(r"((https|http):\\/\\/)?([a-z0-9_\-]+)\.([a-z]{2,6})(\?)?(.+)").unwrap();
  url_regex.is_match(potential_url)
}

pub fn get_metadata(text: &str) -> HashMap<String, String> {
  let mut meta: HashMap<String, String> = HashMap::new();
  for line in text.trim().split("\n") {
    let first_char = line.chars().nth(0).unwrap_or('-');
    if first_char != '!' { continue }

    let is_kv = line.to_string().contains(":");
    if is_kv {
      let parts: Vec<&str> = line.split(":").collect();
      let key = parts[0][1..].to_string();
      let val = parts[1].trim().to_string();

      meta.insert(key, val);
    } else {
      let key = if is_url(&line[1..]) { "URL".to_string() } else { "Description".to_string() };
      let val = line[1..].trim().to_string();
      meta.insert(key, val);
    }
  }

  meta
}

pub fn find_significant_lines(lines: Vec<&str>) -> Vec<&str> {
  lines
  .iter()
  .filter(|l| l.trim() != "".to_string() && (l.contains("O") || l.contains(".")) && !l.contains("!"))
  .map(|l| l.to_owned())
  .collect()
}

pub fn pad_cell_lines(lines: Vec<&str>) -> Vec<String> {
  let largest = find_largest_line(&lines);

  lines
  .iter()
  .map(|line| {
    let needed = largest - line.len();
    format!("{}{}", line, str::repeat(".", needed))
  })
  .collect()
}

pub fn find_largest_line(lines: &Vec<&str>) -> usize {
 lines
 .iter()
 .map(|l| l.len())
 .fold(0, |largest, current| {
    if current > largest {current} else {largest}
 })
}