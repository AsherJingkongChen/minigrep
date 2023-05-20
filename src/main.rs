use std::{env, process};
use minigrep::*;

fn main() {
  let config = Config::parse(env::args())
    .unwrap_or_else(|err| {
      eprintln!("Failed to parse configurations:");
      eprintln!("{}", err);
      process::exit(1);
    });
  run(&config).unwrap_or_else(|err| {
    eprintln!("Failed to run the program:");
    eprintln!("{}", err);
    process::exit(1);
  });
}
