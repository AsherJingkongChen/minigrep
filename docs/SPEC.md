# Requirements

## Description
In the simplest use case, `grep` searches a specified file for a specified string.
To do so, `grep` takes as its arguments a file path and a string.
Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

## Functional
- Takes 2 arguments: a file path and a query string
- Finds lines in the file that contain the query string, and print those lines

## Non-functional
- Learning Rust
- Separate concerns of a binary program
- Unit tests
- Reasonable error handling