# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Run Commands

- **Run all solutions:** `cargo run` (or `cargo run --release` for faster execution)
- **Run specific year:** `cargo run year2024`
- **Run specific day:** `cargo run year2024 day04`
- **Run tests:** `cargo test`
- **Test specific day:** `cargo test year2024::day04`
- **Test specific part:** `cargo test year2024::day04::part2`
- **Benchmarks:** `cargo bench year2024/day04/part2`

## Architecture

Each day's solution follows a consistent pattern with three required functions:
- `parse(input: &str) -> Input` - Parse raw input into a typed structure
- `part1(input: &Input) -> impl Display` - Solve part 1
- `part2(input: &Input) -> impl Display` - Solve part 2

### File Structure

- Solutions: `src/yearXXXX/dayYY.rs`
- Tests: `tests/yearXXXX/dayYY.rs`
- Inputs: `input/yearXXXX/dayYY.txt`
- Templates: `src/template_year/template_day.rs` and `tests/template_year/template_day.rs`

### Adding a New Day

1. Create input file at `input/yearXXXX/dayYY.txt`
2. Create solution at `src/yearXXXX/dayYY.rs` (copy from template)
3. Create test at `tests/yearXXXX/dayYY.rs` (copy from template)
4. Add day to macros in: `src/lib.rs`, `src/main.rs`, `tests/tests.rs`, `benches/benchmarks.rs`

## Utility Modules (`src/utils/`)

- **Grid<T>** - 2D grid with `Point` indexing, iteration, bounds checking, neighbor finding
- **Point** - 2D point with arithmetic ops, direction constants (ORTHOGONALS, DIRS), rotation, normalization
- **Direction** - Clockwise/Anticlockwise rotation enum
- **parsing** - Input parsing helpers

## Code Style

- Uses `clippy::pedantic` (with some casts allowed)
- Tests use example inputs from problem descriptions
- Requires Rust 1.85+ (edition 2024)
