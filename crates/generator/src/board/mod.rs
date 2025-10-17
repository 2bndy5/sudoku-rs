mod traits;
use std::collections::HashSet;

use crate::region::RegionKind;
use crate::{
    BoardSize,
    cell::{Cell, Coord},
};
pub use traits::ParseError;

/// A Sudoku board.
///
/// - Use [`Board::new()`] to create a new board of [`Cell::Notes`].
/// - Use [`Board::default()`] to create a standard 9x9 board of [`Cell::Notes`].
/// - Use [`Board::from_str()`](#impl-FromStr-for-Board) to import a board from a string.
/// - Use [`String::from`](#impl-From%3C%26Board%3E-for-String) to export a board to a string.
///
/// See also [`Board::generate()`], [`Board::solve()`], and [`Board::validate()`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The size of the board.
    ///
    /// This implies maximum number of cells in a row, column, or region.
    pub size: BoardSize,

    /// The type describing the intended shape of the board's regions.
    pub kind: RegionKind,

    /// The grid of cells.
    pub(crate) grid: Vec<Vec<Cell>>,
}

impl Board {
    /// Create a board of [`Cell::Notes`] spanning [`BoardSize::max_cells()`] in
    /// both height and width.
    pub fn new(size: &BoardSize, kind: RegionKind) -> Self {
        let max_cells = size.max_cells();
        let grid = vec![
            vec![Cell::Notes(HashSet::from_iter(0..max_cells)); max_cells as usize];
            max_cells as usize
        ];
        Board {
            size: size.clone(),
            kind,
            grid,
        }
    }

    /// Get the [`Cell`] (by immutable reference) at the given `coordinate`.
    ///
    /// Returns [`None`] if the given `coord` is out of bounds.
    pub fn get_cell(&self, coord: Coord) -> Option<&Cell> {
        let max_cells = self.size.max_cells() as usize;
        if coord.row < max_cells && coord.col < max_cells {
            Some(&self.grid[coord.row][coord.col])
        } else {
            None
        }
    }

    /// Set the value of a cell at the given row and column.
    ///
    /// The `value` must be in range [`0`, [`BoardSize::max_cells()`]).
    /// If the `value` or the `coord` are out of bounds, then nothing is done.
    ///
    /// This also updates the [`Cell::Notes`] of the affected cells in the same
    /// row and column and siblings of the cell's region.
    pub fn set_cell(&mut self, coord: Coord, value: u8) {
        self.set_cell_unchecked_grid(coord, value);
        for sibling in self.kind.siblings(&self.size, coord) {
            if let Cell::Notes(notes) = &mut self.grid[sibling.row][sibling.col] {
                notes.retain(|&n| n != value);
            }
        }
    }

    /// Same as [`Board::set_cell()`] but without checking for outdated notes in the
    /// region containing the cell specified by `coord`.
    ///
    /// Only useful for seeding a board with known valid values in 1 region.
    /// Used internally by [`Board::seed()`].
    pub(super) fn set_cell_unchecked_grid(&mut self, coord: Coord, value: u8) {
        let max_cells = self.size.max_cells() as usize;
        if coord.row < max_cells && coord.col < max_cells && value < max_cells as u8 {
            self.grid[coord.row][coord.col] = Cell::Value(value);
            // Remove notes from other cells in the same row and column.
            for col in &mut self.grid[coord.row] {
                if let Cell::Notes(notes) = col {
                    notes.retain(|&n| n != value);
                }
            }
            for row in 0..max_cells {
                if let Cell::Notes(notes) = &mut self.grid[row][coord.col] {
                    notes.retain(|&n| n != value);
                }
            }
        }
    }

    /// Clear the value of a cell at the given row and column.
    ///
    /// If the specified cell is already empty, then nothing is done.
    ///
    /// This also re-populates the [`Cell::Notes`] of the affected cells in the same
    /// row and column and siblings of the cell's region.
    ///
    /// If making a game with an "undo" feature, then it is recommended to
    /// save/restore the state of the board (via [`Board::clone()`]) instead of
    /// calling this method. This will not restore any invalid (player-made) notes
    /// that were removed when [`Board::set_cell()`] was called.
    pub fn clear_cell(&mut self, coord: Coord) {
        let max_cells = self.size.max_cells() as usize;
        if coord.row >= max_cells || coord.col >= max_cells {
            return;
        }
        if let Cell::Value(value) = self.grid[coord.row][coord.col] {
            // If the cell is cleared, then repopulate the notes for
            // that cell and its related cells.
            self.grid[coord.row][coord.col] = Cell::Notes(HashSet::from_iter([value]));
            let mut not_possible = Vec::new();

            // accumulate filled values in the same region.
            for sibling in self.kind.siblings(&self.size, coord) {
                match &mut self.grid[sibling.row][sibling.col] {
                    Cell::Notes(notes) => {
                        if !notes.contains(&value) {
                            notes.insert(value);
                        }
                    }
                    Cell::Value(v) => {
                        if !not_possible.contains(v) {
                            not_possible.push(v.to_owned());
                        }
                    }
                }
            }

            // accumulate filled values in the same column.
            for i in 0..max_cells {
                match &mut self.grid[i][coord.col] {
                    Cell::Notes(notes) => {
                        if i != coord.row && !notes.contains(&value) {
                            notes.insert(value);
                        }
                    }
                    Cell::Value(v) => {
                        if !not_possible.contains(v) {
                            not_possible.push(v.to_owned());
                        }
                    }
                }
            }

            // accumulate filled values in the same row.
            for (i, cell) in self.grid[coord.row].iter_mut().enumerate() {
                match cell {
                    Cell::Notes(notes) => {
                        if i != coord.col && !notes.contains(&value) {
                            notes.insert(value);
                        }
                    }
                    Cell::Value(v) => {
                        if !not_possible.contains(v) {
                            not_possible.push(v.to_owned());
                        }
                    }
                }
            }

            let notes = (0..max_cells as u8)
                .filter(|v| !not_possible.contains(v))
                .collect();
            self.grid[coord.row][coord.col] = Cell::Notes(notes);
        }
    }

    /// Set a note in the cell at the given coordinate.
    ///
    /// This function does nothing if
    ///
    /// - the specified cell is not a [`Cell::Notes`]
    /// - the [`Cell::Notes`] already contains the given `note`
    /// - the `coord` is out of bounds
    /// - the `note` is out of bounds -- not in range [`0`, [`BoardSize::max_cells()`])
    pub fn set_note(&mut self, coord: Coord, note: u8) {
        let max_cells = self.size.max_cells() as usize;
        if coord.row < max_cells
            && coord.col < max_cells
            && note < max_cells as u8
            && let Cell::Notes(notes) = &mut self.grid[coord.row][coord.col]
            && !notes.contains(&note)
        {
            notes.insert(note);
        }
    }

    /// Clear a note from the cell at the given coordinate.
    ///
    /// This function does nothing if
    ///
    /// - the specified cell is not a [`Cell::Notes`]
    /// - the [`Cell::Notes`] does not contain the given `note`
    /// - the `coord` is out of bounds
    /// - the `note` is out of bounds -- not in range [`0`, [`BoardSize::max_cells()`])
    pub fn clear_note(&mut self, coord: Coord, note: u8) {
        let max_cells = self.size.max_cells() as usize;
        if coord.row < max_cells
            && coord.col < max_cells
            && note < max_cells as u8
            && let Cell::Notes(notes) = &mut self.grid[coord.row][coord.col]
        {
            notes.retain(|&n| n != note);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{
        Board,
        cell::{Cell, Coord},
    };

    #[test]
    fn set_cell() {
        let mut board = Board::default();
        // notes are auto-populated by Board::default()
        let valid = board.size.max_cells() as usize - 1;
        board.set_cell(
            Coord {
                row: valid,
                col: valid,
            },
            valid as u8,
        );
        assert_eq!(board.grid[valid][valid], Cell::Value(valid as u8));
        //check siblings' notes
        for coord in board.kind.siblings(
            &board.size,
            Coord {
                row: valid,
                col: valid,
            },
        ) {
            let notes = board.grid[coord.row][coord.col].get_notes().unwrap();
            assert!(!notes.contains(&(valid as u8)));
        }
        // check note for other cells in the same row
        for (i, cell) in board.grid[valid].iter().enumerate() {
            if i != valid {
                let notes = cell.get_notes().unwrap();
                assert!(!notes.contains(&(valid as u8)));
            }
        }
        // check note for other cells in the same column
        for i in 0..board.size.max_cells() as usize {
            if i != valid {
                let notes = board.grid[i][valid].get_notes().unwrap();
                assert!(!notes.contains(&(valid as u8)));
            }
        }
    }

    #[test]
    fn clear_cell() {
        let mut board = Board::default();
        assert!(board.generate());
        assert!(board.validate(true).is_ok());
        println!("{}", String::from(&board));
        let max_cells = board.size.max_cells() as usize;

        // store expected notes from validated values
        let mut valid_values = Vec::new();
        for row in 0..max_cells {
            let mut row_values = vec![];
            for col in 0..max_cells {
                row_values.push(board.grid[row][col].get_value().unwrap());
                // board.clear_cell(row, col);
            }
            valid_values.push(row_values);
        }

        // clear the cells defined by tested_sets
        for row in 0..max_cells {
            for col in 0..max_cells {
                board.clear_cell(Coord { row, col });
                assert!(board.grid[row][col].is_notes());
            }
        }
        board.clear_cell(Coord::default()); // repeated clear should do nothing

        // check notes for the cleared cells
        for row in 0..max_cells {
            for col in 0..max_cells {
                let notes = &board.grid[row][col].get_notes().unwrap();
                println!("({row}, {col}) notes: {notes:?}");
                assert!(notes.contains(&valid_values[row][col]));
            }
        }
    }

    #[test]
    fn set_note() {
        let mut board = Board::default();
        let valid = board.size.max_cells() as usize - 1;
        let coord = Coord {
            row: valid,
            col: valid,
        };
        // notes are auto-populated by Board::default()
        // so first clear the notes in the cell.
        board.clear_note(coord, valid as u8);
        let notes = &board.grid[valid][valid].get_notes().unwrap();
        assert!(!notes.contains(&(valid as u8)));
        board.set_note(coord, valid as u8);
        let notes = &board.grid[valid][valid].get_notes().unwrap();
        assert!(notes.contains(&(valid as u8)));
    }

    #[test]
    fn invalid_access() {
        let mut board = Board::default();
        let faulty = board.size.max_cells() as usize;
        let valid = faulty - 1;

        assert!(
            board
                .get_cell(Coord {
                    row: faulty,
                    col: valid
                })
                .is_none()
        );
        assert!(
            board
                .get_cell(Coord {
                    row: valid,
                    col: faulty
                })
                .is_none()
        );

        board.set_cell(
            Coord {
                row: faulty,
                col: valid,
            },
            valid as u8,
        );
        board.set_cell(
            Coord {
                row: valid,
                col: faulty,
            },
            valid as u8,
        );
        board.set_cell(
            Coord {
                row: valid,
                col: valid,
            },
            faulty as u8,
        );
        board.clear_cell(Coord {
            row: valid,
            col: faulty,
        });
        board.clear_cell(Coord {
            row: faulty,
            col: valid,
        });
        // notes are auto-populated by Board::default()
        assert_eq!(
            board.grid[valid][valid],
            Cell::Notes(HashSet::from_iter(0..board.size.max_cells()))
        );
    }
}
