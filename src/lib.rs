pub mod search;

use std::{error::Error, fs, env::{self, VarError}};
use search::search;

pub struct Config {
  pub query_string: String,
  pub file_path: String,
}

impl Config {
  pub fn parse(
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
    args.next().ok_or("No argument")?; // Program name
    let query_string = args.next().ok_or(
      "Not enough arguments: missing query string"
    )?;
    if let Some(file_path) = args.next() {
      Ok(Config { query_string, file_path })
    } else {
      Err("Not enough arguments: missing file path")
    }
  }

  // Returns the number of lines that containing the query string
  // If no line is found, return Err
  pub fn run(&self) -> Result<usize, Box<dyn Error>> {
    let query = &self.query_string;
    let lines = fs::read_to_string(&self.file_path)?;
    let ignore_case = env::var("IGNORE_CASE")
      .and_then(|ok| {
        match ok.as_str() {
          "0" => Err(VarError::NotPresent),
          _ => Ok(ok),
        }
      })
      .is_ok();
    let results = search(query, &lines, ignore_case);
    for result in &results {
      println!("{}", result);
    }

    match results.len() {
      0 => Err("Not found".into()),
      number_of_lines => Ok(number_of_lines),
    }
  }
}
