# sudoku-gen

A library that can generate/import, solve, and validate a Sudoku puzzle.

## Features

<!-- markdownlint-disable MD031 MD033 -->

- [x] Import a Sudoku board from a string (`Board::from_str(s: &str)`)
  (for Regular Sudoku boards only -- see [supported regions](#supported-regions) below).
- [x] Export a Sudoku board to a string (`String::from(b: &Board)`)
  (for Regular Sudoku boards only -- see [supported regions](#supported-regions) below).
- [x] Conveniently implemented `Display` trait to
  print a Sudoku `Board` in a human-readable form. (`print!("{board}")`)
- [x] Generate Sudoku boards.
  ```rust
  use sudoku_gen::Board;

  let mut board = Board::default(); // regular 9x9
  assert!(board.generate());
  print!("{board}");
  // 3 7 8 ║ 1 2 4 ║ 5 6 9
  // 6 9 1 ║ 3 5 7 ║ 2 4 8
  // 2 5 4 ║ 6 8 9 ║ 1 3 7
  // ══════╬═══════╬══════
  // 1 2 5 ║ 7 6 3 ║ 8 9 4
  // 7 4 9 ║ 8 1 2 ║ 6 5 3
  // 8 3 6 ║ 9 4 5 ║ 7 1 2
  // ══════╬═══════╬══════
  // 4 1 3 ║ 2 7 6 ║ 9 8 5
  // 5 6 2 ║ 4 9 8 ║ 3 7 1
  // 9 8 7 ║ 5 3 1 ║ 4 2 6
  ```
- [x] Validate Sudoku boards (solved or incomplete).
  ```rust
  use sudoku_gen::Board;
  use std::str::FromStr;

  let mut board = Board::from_str(include_str!("tests/basic.txt")).unwrap();
  print!("{board}");
  // . 5 . ║ 9 . 8 ║ 6 . .
  // 8 . . ║ . . 6 ║ . . 7
  // . . 6 ║ . 2 . ║ . . .
  // ══════╬═══════╬══════
  // . . 9 ║ . . . ║ . 7 .
  // 2 . 3 ║ . . . ║ 8 . 9
  // . 1 . ║ . . . ║ 4 . .
  // ══════╬═══════╬══════
  // . . . ║ . 3 . ║ 7 . .
  // 9 . . ║ 8 . . ║ . . 4
  // . . 5 ║ 6 . 4 ║ . 3 .
  assert!(board.validate(/*solved=*/false).is_ok());
  assert!(board.solve());
  assert!(board.validate(/*solved=*/true).is_ok());
  print!("{board}");
  // 3 5 7 ║ 9 4 8 ║ 6 2 1
  // 8 2 1 ║ 3 5 6 ║ 9 4 7
  // 4 9 6 ║ 7 2 1 ║ 3 8 5
  // ══════╬═══════╬══════
  // 5 4 9 ║ 1 8 3 ║ 2 7 6
  // 2 7 3 ║ 4 6 5 ║ 8 1 9
  // 6 1 8 ║ 2 7 9 ║ 4 5 3
  // ══════╬═══════╬══════
  // 1 6 4 ║ 5 3 2 ║ 7 9 8
  // 9 3 2 ║ 8 1 7 ║ 5 6 4
  // 7 8 5 ║ 6 9 4 ║ 1 3 2
  ```
- [x] Includes a basic executable binary (`sudoku-gen`) for my fellow nerds 🤓
  <details><summary><code>sudoku-gen --help</code></summary>

  ```text
  A tool to generate/import, solve, and validate Sudoku puzzles.

  Usage: sudoku-gen [OPTIONS] [STRING]

  Arguments:
    [STRING]
            Load a board from a string instead of generating a new one.

            Line endings are stripped before parsing.
            The board's size is determined by the length of the
            string after stripping.

            Filled cells are represented using
              1-4 for 4x4 boards
              1-6 for 6x6 boards
              1-9 for 9x9 boards
              0-9 and A-B (or a-b) for 12x12 boards
              0-9 and A-F (or a-f) for 16x16 boards
            Empty cells are represented by any other character
            (including spaces and commas).

  Options:
    -s, --size <SIZE>
            The maximum number of cells in any column or row.

            Ignored if importing a puzzle from a file or string.

            [default: 9]
            [possible values: 4, 6, 9, 12, 16]

    -i, --irregular
            Use an irregular (jigsaw) regions instead of the regular rectangular regions.

            Ignored if importing a puzzle from a file or string.

    -e, --export
            Export the solved board as a string instead of pretty printing it.

            This turns off other console output except for errors.
            This also forces regular regions (nullifies `--irregular`).

    -p, --from-path <FROM_PATH>
            Load a board from a file instead of generating a new one.

            Ignored if importing a puzzle from a string.
            See expectations about the optional argument STRING.

    -h, --help
            Print help (see a summary with '-h')

    -V, --version
            Print version
  ```

  </details>

  Building this from source requires the `bin` feature:

  ```shell
  cargo install sudoku-gen --features bin
  ```

## Supported Sizes

- [x] 4x4 (`BoardSize::X4`)
- [x] 6x6 (`BoardSize::X6` with 3x2 regions)
- [x] 9x9 (`BoardSize::X9`)
- [x] 12x12 (`BoardSize::X12` with 4x3 regions)
- [x] 16x16 (`BoardSize::X16`)

## Supported Regions

"Regions" is the technical term for a grid of related cells within the board.

There is (and will be) no support for overlapping regions.
Given the complexity of this idea,
overlapping regions is considered an undesirable feature.

### `RegionKind::Regular`

Regular regions take a rectangular shape.
Depending on the size of the board (see above),
regular regions are not always perfect squares.

### `RegionKind::Irregular` ("JigSaw")

Irregular regions typically do not form the shape of a rectangle,
rather they often resemble the shape of Tetris pieces.
Depending on how each cell is mapped to a region,
it is possible for an irregular region to form a rectangle.

Irregular regions can be generated (`IrregularMap::generate()`) or
explicitly mapped (`IrregularMap::new()`).
In either case, the map is validated to ensure

- all rows' and columns' length match the given `BoardSize`.
- all mapped regions conform to the given `BoardSize`. Meaning

  - the region index does not exceed the `BoardSize`
  - each region is contiguous -- each cell of a region connects perpendicularly to
    at least one other cell of the same region.
  - each region has the correct number of cells (based on `BoardSize`)

## Strategies

The solving algorithm (recursive backtracking) implements the
following techniques (in order of execution):

- "naked singles": When only 1 candidate for a cell remains, set the cell to
  that candidate.
- "hidden singles": When only 1 cell in a region, row, or column can be a certain
  candidate (hidden among other candidates for that cell), set the cell to that
  candidate.
- "pointed pair/triple/set": When a candidate is deduced to be in a single row or
  column of a region, then that candidate is removed from all other unsolved
  cells in the same row or column of other regions.
- "hidden pair/triple/set": When a row or column has finite number of cells that
  share the same candidates, then eliminate the candidates that are not shared among
  the cells with shared candidates.
- The `Board::set_cell()` method keeps cells updated similar to how a human would
  keep notes on empty cells. This ensures that the candidates for each unfilled cell
  are compliant with sudoku logic (before applying the strategies listed above).

For a better explanation (with visual examples) of various strategies,
please review the excellent [sudoku.coach] website. Note, that website
confusingly refers to regions as "boxes".

Other strategies may be added when I get the time.
These are just the strategies I use to play Sudoku 😉.

[sudoku.coach]: https://sudoku.coach/en/learn/technique-overview
