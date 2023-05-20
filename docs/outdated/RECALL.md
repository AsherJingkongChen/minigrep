# Points to recall (Outdated)

1. `if let` expression
2. `if` in `let` statement
3. Lifetime annotation
   - ```rust
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
        |
      1 ~ pub fn search<'a>(
      2 ~   query: &'a str,
      3 ~   lines: &'a str,
      4 |   ignore_case: bool,
      5 ~ ) -> Vec<&'a str> {
        |
     ```
   - [Reference](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-function-signatures)
4. Trait object
   - `Box<dyn Error>`
   - if an object walks like a duck and quacks like a duck, it may as well be a duck
   - [Reference](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#implementing-the-trait)
5. Iterator
   - `impl Iterator<Item = T>`
   - [Reference](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html#using-the-returned-iterator-directly)
6. Module
   - `pub mod search;`
   - `use minigrep::Config;`
   - `use std::{error::Error, fs, env::{self, VarError}};`
     - Less ambiguity
7. Error handling
   - [Reference](https://slidev.andyjjrt.cc/rust-ch9/14)
   - [Reference](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling)
9. Testing
   - Note the #[test] annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
   - [Reference](https://doc.rust-lang.org/book/ch11-01-writing-tests.html#testing-equality-with-the-assert_eq-and-assert_ne-macros)