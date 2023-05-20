// Environment variables are not thread-safe.
// To eliminate side effects, integration tests that depends on
// different values of a environment variable are isolated.

#[cfg(test)]
mod config {
  mod run {
    use std::env;
    use minigrep::Config;

    #[test]
    fn search_error_ignore_case_in_rust_book_ch12_04() {
      let config = Config::parse([
        "<program_name>".into(),
        "eRRor".into(),
        "./tests/assets/rust_book_ch12_04.txt".into()
      ].into_iter());
      assert!(config.is_ok());
      let config = config.unwrap();

      env::set_var("IGNORE_CASE", "1");
      assert!(env::var("IGNORE_CASE").is_ok());

      let result = config.run();
      assert!(result.is_ok());
      let result = result.unwrap();
      assert_eq!(result, 19);
    }
  }
}