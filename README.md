# Advent of Code 2025 - Rust Solutions

This repository contains my solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Project Structure

```
AOC2025/
├── Cargo.toml          # Rust project configuration
├── Input/              # Puzzle input files (not committed to git)
│   ├── day01.txt
│   ├── day02.txt
│   └── ...
└── src/
    ├── lib.rs          # Shared utilities and helper functions
    └── bin/
        ├── day01.rs    # Day 1 solution
        ├── day02.rs    # Day 2 solution
        └── ...
```

## Setup

1. Make sure you have [Rust installed](https://rustup.rs/)
2. Clone this repository
3. Add your puzzle inputs to the `Input/` folder with the naming convention `dayXX.txt` (e.g., `day01.txt`, `day02.txt`)

## Usage

### Running a solution

To run the solution for a specific day:

```bash
cargo run --bin day01
```

### Running tests

Each day's solution includes tests based on the example inputs from the puzzle.

To run tests for a specific day:

```bash
cargo test --bin day01
```

To run all tests:

```bash
cargo test
```

### Adding a new day

1. Copy `src/bin/day01.rs` to `src/bin/dayXX.rs` (replace XX with the day number, e.g., `day05.rs`)
2. Update the `read_input(1)` call to `read_input(XX)` in the `main()` function
3. Add your puzzle input to `Input/dayXX.txt`
4. Update the `EXAMPLE_INPUT` constant in the tests with the example from the puzzle
5. Implement `part1()` and `part2()` functions
6. Update the test assertions with expected results from the example

### Building in release mode

For optimal performance:

```bash
cargo build --release
cargo run --release --bin day01
```

## Utility Functions

The `src/lib.rs` module provides helpful utilities:

- `read_input(day: u8)` - Read input file for a specific day
- `read_input_from_path(path)` - Read input from a custom path
- `parse_lines(input)` - Parse input into lines (filtering empty lines)
- `parse_all_lines(input)` - Parse input into lines (keeping empty lines)

## Notes

- Puzzle inputs are personal and should not be committed to version control (they're in `.gitignore`)
- Each day is a separate binary for faster compilation and easier testing
- Tests use the example inputs provided in each day's puzzle description
- Made with help from Cursor


