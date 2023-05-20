pub fn search<'a>(
  query: &'a str,
  lines: &'a str,
  ignore_case: bool,
) -> Vec<&'a str> {
  let mut result = Vec::new();
  let query = if ignore_case {
    query.to_lowercase()
  } else {
    query.to_string()
  };
  for line in lines.lines() {
    let found = if ignore_case {
      line.to_lowercase().contains(&query)
    } else {
      line.contains(&query)
    };
    if found {
      result.push(line);
    }
  }
  result
}
