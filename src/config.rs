pub struct Config {
  pub query_string: String,
  pub file_path: String,
}

impl Config {
  pub fn parse(
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
    args.next().ok_or("No argument")?; // Program name
    let query_string: String = args.next().ok_or(
      "Not enough arguments: missing query string"
    )?;
    // This if let expression is equivalent to Option::ok_or
    if let Some(file_path) = args.next() {
      Ok(Config { query_string, file_path })
    } else {
      Err("Not enough arguments: missing file path")
    }
  }
}
