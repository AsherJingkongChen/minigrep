---
codeslide:
  version: 0.14.0
  format: html
  codeFont:
    family: Source Code Pro
    rule: https://fonts.googleapis.com/css2?family=Source+Code+Pro:wght@300;400;700&display=swap
    size: 0.85em
  slideFont:
    family: Open Sans
    rule: https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,400;0,700;1,400;1,700&display=swap
    size: 1.2rem
  styles:
    - https://doc.rust-lang.org/book/tomorrow-night.css

---
# Minigrep

The URL of GitHub Repository is [here](https://github.com/AsherJingkongChen/minigrep.git)

---
# What should we learn in this project? (1)

Excerpted from [Rust-lang book ch12-00](https://doc.rust-lang.org/book/ch12-00-an-io-project.html):

Our grep project will combine a number of concepts you’ve learned so far:

- Organizing code (using what you learned about modules in Chapter 7)
- Using vectors and strings (collections, Chapter 8)
- Handling errors (Chapter 9)
- Using traits and lifetimes where appropriate (Chapter 10)
- Writing tests (Chapter 11)

---
# What should we learn in this project? (2)

In conclusion, this project is a recap of some chapters in rust-lang book.

---
[:slide](./SPEC.md)

---
# Project layout (1)

Cargo builds source codes from `src`
and test codes from `tests`:
```
./
|-- src/
`-- tests/
```

---
# Project layout (2)

We split the program into a `main.rs` and a `lib.rs` [(link)](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects):
- `main.rs` handles running the program
- `lib.rs` handles all the logics
- This structure is easy to write testing codes
  - Treat test codes as several executables like `main.rs`
- Check [wikipedia](https://en.wikipedia.org/wiki/Separation_of_concerns) (Separation of concerns) if needed

```diff
./
|-- src/
+   |-- lib.rs
+   `-- main.rs
`-- tests/
```

---
# Project layout (3)

The features of program in a nutshell: **"Parse arguments, search the string, and display all the lines containing it."**

We can write 3 modules to complete the tasks:
1. `config.rs`: Store the parsed arguments
2. `search.rs`: Search the string in lines of the file
3. `run.rs`: Do step `2`. Then display all the lines containing the query string.

```diff
./
|-- src/
+   |-- config.rs
|   |-- lib.rs
|   |-- main.rs
+   |-- run.rs
+   `-- search.rs
`-- tests/
```

---
# Project layout (4)

To make sure that the program works, we need some testing codes:
```diff
./
|-- src/
|   |-- config.rs
|   |-- lib.rs
|   |-- main.rs
|   |-- run.rs
|   `-- search.rs
`-- tests/
+   |-- assets/
+   |   `-- rust_book_ch12_04.txt
+   |-- int_tests_1.rs
+   |-- int_tests_2.rs
+   `-- unit_tests.rs
```

We will use unit tests for parsing and searching, and integration tests for running the program.

---
# Parsing arguments (1)

## `std::env::args`
- Returns an iterator over arguments captured from the command

```rs
let args: impl Iterator<Item = String> = std::env::args();
```

---
# Use the iterator (1)

Iterators can be collected into a collection:

```rs
std::env::args().collect::<Vec<String>>()
```

What's bad of it? It **wastes** of memories.

Consider this, there's a lot of useless strings (they are ignored though):
```sh
minigrep string file useless_option_1 ... useless_option_100
```

Or the program may want to exit early when receiving too many arguments:
```
<< minigrep string file useless_option_1 ... useless_option_100
>> Failed to parse:
>> Too many arguments
```

---
# Use the iterator (2)

We only need 2 arguments: the query string and the file path.

Consider this:
```sh
minigrep query_string file_path useless_option
```

The collected vector will be:
```rs
[
  "minigrep",
  "query_string",
  "file_path",
  "useless_option"
]
```

Let's capture them:
```rs
args.next(); // Program name is dropped
let query_string: Option<String> = args.next();
let file_path: Option<String> = args.next();
```

Iterators are just like flowing somen!:

![](https://media.timeout.com/images/105462295/750/422/image.jpg)

---
# Parsing arguments (2)

Let's finish parsing arguments!

- The 2 resolved arguments will be stored in an `Config` object 
- `Option::ok_or`: converts `Option` to `Result`

`src/config.rs`:
[:code.rs](../src/config.rs)

---
# Let's recap from `Config::parse`
> [Back](#parsing-arguments-2)

The `?` operator [(link)](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
- The function returns immediately if the `?` operator matches `Result::Err`

```rs
pub fn parse(
  mut args: impl Iterator<Item = String>,
) -> Result<Config, &'static str> {
  args.next().ok_or("No argument")?; // Program name
  // ...
}
```

---
# Search the string (1)

Consider that we want to search `He` in these lines:
```
Ok google
Hey Bixby
Hello Siri
Hi Alexa
```

The green results will be displayed on the command lines:
```diff
- Ok google
+ Hey Bixby
+ Hello Siri
- Hi Alexa
```

We've found all the lines containing `He` here.

If the string is not found, the message should be like:
```sh
Failed to run the program:
Not found
```

---
# Search the string (2)

- Convert all strings to lower case if `ignore_case` is true
- `str::lines` returns an iterator over the lines of a string

`src/search.rs`:
[:code.rs](../src/search.rs)

---
# Let's recap from `search` (1)
> [Back](#search-the-string-2)

`if` in `let` statement [(link)](https://doc.rust-lang.org/reference/expressions/if-expr.html)

- It works as ternary operator (`?:`) in Rust *(I like it)*

1. ```rs
   let query = if ignore_case {
     query.to_lowercase()
   } else {
     query.to_string()
   };
   ```

2. ```rs
   let found = if ignore_case {
     line.to_lowercase().contains(&query)
   } else {
     line.contains(&query)
   };
   ```

---
# Let's recap from `search` (2)
> [Back](#search-the-string-2)

Lifetime annotation [(link)](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures)

*Wait, why do we need this?*

Consider all the `'a` are removed:
```rs
pub fn search(
  query: &str,
  lines: &str,
  ignore_case: bool,
) -> Vec<&str>
```

The compiler complains:
```rs
    error[E0106]: missing lifetime specifier
--> src/search.rs:5:10
  |
2 |   query: &str,
  |          ----
3 |   lines: &str,
  |          ----
4 |   ignore_case: bool,
5 | ) -> Vec<&str> {
  |          ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `lines`
help: consider introducing a named lifetime parameter
```

---
# Let's recap from `search` (3)
> [Back](#search-the-string-2)

What does the compiler [mean](#lets-recap-from-search-2)?

Let's figure it out:
1. `search` borrows two strings: `query` and `lines`
2. `search` returns some strings from `lines.lines()`

There's an ambiguity: **"who owns the returned strings?"**

The compiler suggests we add lifetime annotations like this:
```rs
pub fn search<'a>(
  query: &'a str,
  lines: &'a str,
  ignore_case: bool,
) -> Vec<&'a str>
```

Actually, we don't need to specify the lifetime of `query`, because the result is regardless of `query`:
```diff
pub fn search<'a>(
-  query: &'a str,
+  query: &str,
  lines: &'a str,
  ignore_case: bool,
) -> Vec<&'a str>
```

---
# Display all the lines containing it

`run` does also [search the string](#search-the-string-2)

`src/run.rs`:
[:code.rs](../src/run.rs)

---
# Let's recap from `run` (1)
> [Back](#display-all-the-lines-containing-it)

`use` brings codes of `crate` into the scope:
```rs
use crate::{
  config::Config,
  search::search,
};
```

`crate` refers to [`src` level](#project-layout-3)

---
# Environment variables

## `std::env::var("IGNORE_CASE")`
- reads the environment variable called `IGNORE_CASE`
- returns `Result`
  - if the variable is not present, return `Err(VarError::NotPresent)`
  - We make it returns `Err` if `IGNORE_CASE` is `"0"`

```rs
// Convert all strings to lower case if IGNORE_CASE presents
let ignore_case: bool = env::var("IGNORE_CASE")
  .and_then(|ok| {
    match ok.as_str() {
      "0" => Err(env::VarError::NotPresent),
      _ => Ok(ok),
    }
  })
  .is_ok();
```

Actually, I prefer use `--` to specify options like `--ignore-case`

To parse CLI arguments in that way, we can use some external crates like [`clap`](https://docs.rs/clap/latest/clap/)

---
# Trait object

Notice what does [`run`](#display-all-the-lines-containing-it) return:
```rs
pub fn run(config: &Config) -> Result<usize, Box<dyn Error>>
```

`Box<dyn TraitName>` is called trait object

*Wait again, why do we need this*?

- `run` returns `Result<usize, Err>` where `Err` can be `std::io::Error` or `&'static str`
- To fit in the result type, we make use of the pointer type `Box` and `Error` trait

Trait object is one of [dynamically sized types](https://doc.rust-lang.org/reference/dynamically-sized-types.html) in Rust, DST is just like abstract class and allocator in C++

---
# `lib.rs`

Bringing modules into the public scope:
[:code.rs](../src/lib.rs)

---
# Unit tests

To make sure each unit of code works, we should write unit tests:
- `Config::parse`
- `search`
- use `cargo test --test unit_test` to run `unit_tests.rs` only:
  ```rs
  running 8 tests
  test config::parse::err ... ok
  test config::parse::ok ... ok
  test search::get_no_line ... ok
  test search::get_no_line_ignore_case ... ok
  test search::get_one_line ... ok
  test search::get_one_line_ignore_case ... ok
  test search::get_two_lines ... ok
  test search::get_two_lines_ignore_case ... ok

  test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```
`tests/unit_tests.rs`:
[:code.rs](../tests/unit_tests.rs)

---
# Integration tests

Units of code that work correctly on their own could have problems when integrated

- use `cargo test -- --show-output` to see displayed messages

`tests/int_tests_1.rs`:
[:code.rs](../tests/int_tests_1.rs)

`tests/int_tests_2.rs`:
[:code.rs](../tests/int_tests_2.rs)

---
# `main.rs`

The entry point of CLI executable

- `use minigrep::*`: minigrep refers to `lib.rs`
- Do all the tasks:
  - [parse arguments](#parsing-arguments-2)
  - [search the string](#search-the-string-2)
  - [display all the lines containing it](#display-all-the-lines-containing-it)
- Reasonable error handling:
  - If `Result::Err` received, display the message and exit with non-zero code
  - `eprintln!(fmt, ...)` passes messages to `stderr`. Remove the `e` if you want to pass to `stdout`

[:code.rs](../src/main.rs)

---
# Commands
- `cargo build`
- `cargo test`
- `cargo run -- <query_string> <file_path>`
- environment variables:
  - `IGNORE_CASE`: whether to ignore case when searching for the string

Reference to [testing with cargo](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#testing-equality-with-the-assert_eq-and-assert_ne-macros)

---
# Thanks for watching!
The URL of GitHub Repository is [here](https://github.com/AsherJingkongChen/minigrep.git)
