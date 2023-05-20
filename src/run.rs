use crate::{
  config::Config,
  search::search,
};
use std::{error::Error, fs, env};

// Returns the number of lines that containing `config.query_string`
// If no line is found, returns Err("Not found")
pub fn run(config: &Config) -> Result<usize, Box<dyn Error>> {
  let query = &config.query_string;
  let lines: String = fs::read_to_string(&config.file_path)?;
  let ignore_case: bool = env::var("IGNORE_CASE")
    .and_then(|ok| {
      match ok.as_str() {
        "0" => Err(env::VarError::NotPresent),
        _ => Ok(ok),
      }
    })
    .is_ok();
  let results: Vec<&str> = search(query, &lines, ignore_case);
  for result in &results {
    println!("{}", result);
  }

  match results.len() {
    0 => Err("Not found".into()),
    number_of_lines => Ok(number_of_lines),
  }
}