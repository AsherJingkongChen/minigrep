#[cfg(test)]
mod config {
  mod run {
    use std::env;
    use minigrep::*;

    #[test]
    fn search_vector_in_rust_book_ch12_04() {
      let config = Config::parse([
        "<program_name>".into(),
        "vector".into(),
        "./tests/assets/rust_book_ch12_04.txt".into()
      ].into_iter());
      assert!(config.is_ok());
      let config = config.unwrap();

      env::set_var("IGNORE_CASE", "0");

      let result = run(&config);
      assert!(result.is_ok());
      let result = result.unwrap();
      assert_eq!(result, 7);
    }

    #[test]
    fn search_error_in_rust_book_ch12_04() {
      let config = Config::parse([
        "<program_name>".into(),
        "error".into(),
        "./tests/assets/rust_book_ch12_04.txt".into()
      ].into_iter());
      assert!(config.is_ok());
      let config = config.unwrap();

      env::set_var("IGNORE_CASE", "0");

      let result = run(&config);
      assert!(result.is_ok());
      let result = result.unwrap();
      assert_eq!(result, 13);
    }
  }
}
