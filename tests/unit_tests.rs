#[cfg(test)]
mod config {
  mod parse {
    use minigrep::Config;

    #[test]
    fn ok() {
      let config = Config::parse([
        "<program_name>".into(),
        "<query_string>".into(),
        "<file_path>".into(),
      ].into_iter());
      assert!(config.is_ok());
      let config = config.unwrap();
      assert_eq!(config.query_string, "<query_string>");
      assert_eq!(config.file_path, "<file_path>");
    }

    #[test]
    fn err() {
      assert!(Config::parse([
      ].into_iter()).is_err());

      assert!(Config::parse([
        "<program_name>".into(),
      ].into_iter()).is_err());

      assert!(Config::parse([
        "<program_name>".into(),
        "<query_string>".into(),
      ].into_iter()).is_err());
    }
  }
}

#[cfg(test)]
mod search {
  use minigrep::search::search;

  #[test]
  fn get_no_line() {
    let result = search(
"okokok", 
"\
ok ok ok
o k o k o k
okokOK
ok
ok
ok",
false);
    assert!(result.is_empty());
  }

  #[test]
  fn get_one_line() {
    let result = search(
"ok okok",
"\
ok ok ok
okok ok
Ok OkOk
ok ok okokok",
false);
    assert_eq!(result, vec![
      "ok ok okokok"
    ]);
  }

  #[test]
  fn get_two_lines() {
    let result = search(
"ok okok",
"\
ok ok ok
kok okok
ok oko
k
Ok OkOk
ok ok okokok",
false);
    assert_eq!(result, vec![
      "kok okok",
      "ok ok okokok"
    ]);
  }

  #[test]
  fn get_no_line_ignore_case() {
    let result = search(
"oKokOK", 
"\
ok ok ok
o k o k o k
okoko k
ok
ok
ok",
true);
    assert!(result.is_empty());
  }

  #[test]
  fn get_one_line_ignore_case() {
    let result = search(
"oK okOk",
"\
ok ok ok
oKok ok
okok
ok OK OkokOk",
true);
    assert_eq!(result, vec![
      "ok OK OkokOk"
    ]);
  }

  #[test]
  fn get_two_lines_ignore_case() {
    let result = search(
"ok OkoK",
"\
ok ok ok
kOK oKOk
ok oko
k
ok
okok
OK OK OkOkOk",
true);
    assert_eq!(result, vec![
      "kOK oKOk",
      "OK OK OkOkOk"
    ]);
  }
}