use std::{fmt::Display, str::FromStr};
use thiserror::Error;

use crate::{Board, BoardKind, BoardSize, Cell, Coord};

impl Display for Board {
    /// Display the board in a human-readable format.
    ///
    /// Cells with no value are represented by `.`.
    /// Filled cells are represented using
    ///   * 1-4 for 4x4 boards
    ///   * 1-6 for 6x6 boards
    ///   * 1-9 for 9x9 boards
    ///   * 0-9 and A-B for 12x12 boards
    ///   * 0-9 and A-F for 16x16 boards
    ///
    /// The cells' value is always in uppercase if it is a hexadecimal digit.
    ///
    /// For irregular Sudoku boards, the region index is shown next to
    /// each cell (in a lowercase hexadecimal digit).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_cells = self.size.max_cells();
        let zero_based_out = (max_cells <= 9) as u32;
        match self.kind {
            BoardKind::Regular => {
                let row_width = self.size.width();
                let col_height = self.size.height();
                for (i, row) in self.grid.iter().enumerate() {
                    if (i as u8).is_multiple_of(col_height) && i != 0 {
                        for _ in 0..((max_cells / row_width) - 1) {
                            write!(f, "{}", "═".repeat(row_width as usize * 2))?;
                            write!(f, "╬═")?;
                        }
                        writeln!(f, "{}", "═".repeat((row_width as usize * 2) - 1))?;
                    }
                    for (j, cell) in row.iter().enumerate() {
                        let suffix = if j == (max_cells as usize - 1) {
                            ""
                        } else {
                            " "
                        };
                        if (j as u8).is_multiple_of(row_width) && j != 0 {
                            write!(f, "║{suffix}")?;
                        }
                        match cell {
                            Cell::Value(v) => write!(
                                f,
                                "{} ",
                                char::from_digit(*v as u32 + zero_based_out, 16)
                                    .unwrap()
                                    .to_ascii_uppercase()
                            )?,
                            Cell::Notes(_) => write!(f, ".{suffix}")?,
                        }
                    }
                    writeln!(f)?;
                }
            }
            BoardKind::Irregular { map: _ } => {
                for (i, row) in self.grid.iter().enumerate() {
                    for (j, cell) in row.iter().enumerate() {
                        let grid_index = self
                            .kind
                            .index(&self.size, Coord { row: i, col: j })
                            .map(|m| char::from_digit(m as u32 + zero_based_out, 16).unwrap())
                            .unwrap_or('?');
                        match cell {
                            Cell::Value(v) => write!(
                                f,
                                "{}┆{grid_index}",
                                char::from_digit(*v as u32 + zero_based_out, 16)
                                    .unwrap()
                                    .to_ascii_uppercase()
                            )?,
                            Cell::Notes(_) => write!(f, ".┆{grid_index}")?,
                        }
                        if j < (max_cells as usize - 1) {
                            write!(f, " │ ")?;
                        } else if i < (max_cells as usize - 1) {
                            writeln!(f, "\n{}───", "────┼─".repeat(max_cells as usize - 1))?
                        } else {
                            writeln!(f)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

/// An error that can occur when parsing a string representation of a board.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ParseError {
    #[error("The string length ({0}) is not a perfect square.")]
    /// The string length is not a perfect square.
    InvalidLength(usize),

    #[error("The board size ({0}x{0}) is not supported.")]
    /// The board size is not supported.
    InvalidSize(usize),
}

impl FromStr for Board {
    type Err = ParseError;

    /// Convert a string representation of a board to a [`Board`].
    ///
    /// The [`BoardSize`] size is inferred from the length of
    /// the string after stripping all line endings.
    ///
    /// Only [`BoardKind::Regular`] boards are supported by this method
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let src = s.replace(['\n', '\r'], "");
        let len = src.chars().count();
        let root = (len as f32).sqrt();
        if root.fract() != 0.0 {
            return Err(ParseError::InvalidLength(len));
        }
        if ![4, 6, 9, 12, 16].contains(&(root as u8)) {
            return Err(ParseError::InvalidSize(root as usize));
        }
        let size = BoardSize::from(root as u8);
        let zero_is_disallowed = size.max_cells() <= 9;
        let max_cells = size.max_cells() as usize;
        let mut board = Board::new(&size, BoardKind::Regular);
        for (i, c) in src.char_indices() {
            let row = i / max_cells;
            let col = i % max_cells;
            if let Some(mut digit) = c.to_digit(16) {
                if zero_is_disallowed {
                    // Only 12x12 and 16x16 boards allow 0 (hex digits).
                    // All other boards use 1 based indexing.
                    // Internally, we use zero based indexing.
                    // So, we need to subtract 1 from the digit,
                    // employing wraparound for 0 - 1.
                    (digit, _) = digit.overflowing_sub(1);
                }
                if digit >= max_cells as u32 {
                    continue;
                }
                board.set_cell(Coord { row, col }, digit as u8);
            }
        }
        Ok(board)
    }
}

impl From<&Board> for String {
    /// Convert a [`Board`] to a string representation.
    ///
    /// This is considered an export function for external verification.
    ///
    /// Cells with no value are represented by `.`.
    /// Filled cells are represented using
    ///   * 1-4 for 4x4 boards
    ///   * 1-6 for 6x6 boards
    ///   * 1-9 for 9x9 boards
    ///   * 0-9 and A-B (or a-b) for 12x12 boards
    ///   * 0-9 and A-F (or a-f) for 16x16 boards
    ///
    /// The cells' value is always in uppercase if it is a hexadecimal digit.
    fn from(board: &Board) -> Self {
        let mut s = String::new();
        let zero_is_disallowed = (board.size.max_cells() <= 9) as u8;
        for row in &board.grid {
            for cell in row {
                match cell {
                    // unwrap() is safe because value is guaranteed to be a hex digit.
                    Cell::Value(v) => s.push(
                        char::from_digit((v + zero_is_disallowed) as u32, 16)
                            .unwrap()
                            .to_ascii_uppercase(),
                    ),
                    Cell::Notes(_) => s.push('.'),
                }
            }
        }
        s
    }
}

impl Default for Board {
    /// Create a standard 9x9 board.
    fn default() -> Self {
        Self::new(&BoardSize::X9, BoardKind::Regular)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Board, BoardKind, IrregularMap, ParseError};

    #[test]
    fn print_irregular_board() {
        let input = "1034.2....3... 4";
        let kind = BoardKind::Irregular {
            map: IrregularMap::new(vec![
                vec![0, 0, 1, 1],
                vec![0, 1, 1, 3],
                vec![0, 2, 2, 3],
                vec![2, 2, 3, 3],
            ])
            .unwrap(),
        };
        let mut board = Board::from_str(input).unwrap();
        board.kind = kind;
        println!("{board}");
        let output = String::from(&board);
        assert_eq!(input.replace(&['0', ' '], "."), output);
    }

    #[test]
    fn print_4x4_board() {
        let input = concat!("1034", ".2..", "..3.", "...4");
        let board = Board::from_str(input).unwrap();
        let display = format!("{board}");
        println!("{display}");
        assert!(!display.contains('0'));
        let output = String::from(&board);
        assert_eq!(input.replace(&[' ', '0'], "."), output);
    }

    #[test]
    fn print_6x6_board() {
        let input = "1.34.6...0 ...3.5........5.3..6.34.1";
        let board = Board::from_str(input).unwrap();
        let display = format!("{board}");
        println!("{display}");
        assert!(!display.contains('0'));
        let output = String::from(&board);
        assert_eq!(input.replace(&[' ', '0'], "."), output);
    }

    #[test]
    fn print_9x9_board() {
        let input = concat!(
            "53..7....",
            "6..195. 0",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79"
        );
        let board = Board::from_str(input).unwrap();
        let display = format!("{board}");
        println!("{display}");
        assert!(!display.contains('0'));
        let output = String::from(&board);
        assert_eq!(input.replace(&[' ', '0'], "."), output);
    }

    #[test]
    fn print_12x12_board() {
        let input = concat!(
            ".23456789ABCD0E1 ..3456789abcd0E1.2 ..456789ABCD0E12.3 ",
            "..56789ABCD0E123.4 ..6789ABCD0E1234.5 ..789ABCD0E12346 ",
            "..89ABCD0E123456. ..9ABCD0E1234567",
        );
        let board = Board::from_str(input).unwrap();
        let display = format!("{board}");
        println!("{display}");
        for illegal in ['C', 'D', 'E', 'F'] {
            assert!(!display.contains(illegal));
        }
        let output = String::from(&board);
        assert_eq!(
            input
                .to_ascii_uppercase()
                .replace(&[' ', 'C', 'D', 'E', 'F'], "."),
            output
        );
    }

    #[test]
    fn print_16x16_board() {
        let input = concat!(
            "1..A..B..C..D..E..F..0..2..3..4..5..6..7..8..9.. ",
            ".23456789ABCDEF01 ..3456789ABCDEF01.2 ..456789ABC",
            "DEF012.3 ..56789ABCDEF0123.4 ..6789ABCDEF01234.5 ",
            "..789ABCDEF012345.6 ..89ABCDEF0123456.7 ..9ABCDEF",
            "01234567.8 ..ABCDEF012345678.9 ..BCDEF0123456789.",
            "A ..CDEF012",
        );
        let board = Board::from_str(input).unwrap();
        println!("{board}");
        let output = String::from(&board);
        assert_eq!(input.replace(' ', ".").to_ascii_uppercase(), output);
    }

    #[test]
    fn parse_invalid() {
        assert_eq!(
            Board::from_str("invalid string"),
            Err(ParseError::InvalidLength(14))
        );
        assert_eq!(Board::from_str("1234"), Err(ParseError::InvalidSize(2)));
    }
}
