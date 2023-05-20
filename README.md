# Minigrep

## Acknowlegement

The repo is for learning [The Rust Programming Language Book - Ch12. An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

## Prerequisites
- cargo (Rust package manager)

## Documents
- [Specifications](./docs/SPEC.md)
- [Slideshow](https://asherjingkongchen.github.io/minigrep) (Built with [CodeSlide](https://github.com/AsherJingkongChen/codeslide))

## Commands
- `cargo build`
- `cargo test`
- `cargo run -- <query_string> <file_path>`
- environment variables:
  - `IGNORE_CASE`: whether to ignore case when searching for the string
