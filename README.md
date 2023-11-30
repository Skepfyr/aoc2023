# [Advent of Code 2023](https://adventofcode.com/2023) Solutions in [Rust](https://www.rust-lang.org/)

Who knows why I'm doing this again.

## Running
If you want to run these solutions against your own input then you can:
```bash
# Run a specific day's solution against input in a file.
cargo run -- 1 input.txt
# Run a specific day's solution against input from stdin.
cargo run -- 24 -
# Run a specific day's solution against the corresponding input in input/dayXX.txt
cargo run -- 5
```
If `input/dayXX.txt` doesn't exist (and you run the version that tries to read from that file), then it will attempt to download your input from advent of code using the credentials passed in `--session-cookie` or in the `AOC_SESSION_COOKIE` environment variable.
