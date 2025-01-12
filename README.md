# Calendar Puzzle Solver

This is a CLI tool to solve the calendar puzzle ["Calendarium"][calendarium] by Dmitry Andreev (Pluton).

[calendarium]: https://www.youtube.com/watch?v=FCt1SOVM--0

![Screenshot of the tool solving the puzzle](https://github.com/user-attachments/assets/e07de674-e409-4aa3-8f64-3eab6eee9b36)

Calendarium is a puzzle that shows all of the months of the year, the days of the month, and the days of the week, on a 6×9 grid (with the bottom-right corner missing). The goal is to use ten "pentomino" pieces to cover the entire puzzle board, except for the weekday, month day, and month of the current calendar day.

Since there are 366 possible days of the year, and each of them could be any of the seven days of the week, there are a total of 2,562 individual challenges in this one puzzle. But there are often dozens of unique solutions for a given day.

I created this tool for fun, and to help me learn more about Rust.

It is currently not using the most efficient method of solving this type of puzzle, which would be Donald Knuth's "Dancing Links" algorithm. Instead, it uses a more standard backtracking algorithm, but with some optimizations to make it faster.

## Installation

`cargo build --release`

This will create a binary in `target/release/calendar_puzzle`.

## Usage

`calendar_puzzle` — Start the program, which will prompt you for a date to solve, and find the first solution

### Flags

`--today` — Find first solution for today's date (no date prompt)
`--all` — Find all solutions instead of just the first one
`--show-pieces` — Show the pieces before solving

### Notes

If you are not asking for all solutions, the program will stop after finding the first one. Because it uses multiple threads on your computer to find solutions, it may not always find the same solution first, even for the same date.

