use std::collections::HashMap;

use crate::{Board, Cell, Coord, cell::ValidatedCell};

/// Errors that can occur during board validation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidationResults {
    /// A mapping of duplicated values to the corresponding coordinates.
    ///
    /// This map accounts for duplicate values in a row, column, or region.
    pub duplicates: HashMap<u8, Vec<Coord>>,

    /// Cell(s) that are not filled when using
    /// [`board.validate(true)`](fn@Board::validate).
    ///
    /// This is not populated if using
    /// [`board.validate(false)`](fn@Board::validate).
    pub incomplete: Vec<Coord>,

    /// Cell(s) that contains a value that is not less than
    /// the number of regions, column, or rows.
    pub invalid_values: Vec<Coord>,
}

impl ValidationResults {
    fn push_duplicate(&mut self, value: u8, coords: Vec<Coord>) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.duplicates.entry(value) {
            e.insert(coords);
        } else {
            let dupe_list = self
                .duplicates
                .get_mut(&value)
                // safe to unwrap() because we checked with `contains_key()`
                .unwrap();
            for coord in coords {
                if !dupe_list.contains(&coord) {
                    dupe_list.push(coord);
                }
            }
        }
    }

    fn push_incomplete(&mut self, coord: Coord) {
        if !self.incomplete.contains(&coord) {
            self.incomplete.push(coord);
        }
    }

    fn push_invalid_value(&mut self, coord: Coord) {
        if !self.invalid_values.contains(&coord) {
            self.invalid_values.push(coord);
        }
    }

    /// True if the board is valid.
    ///
    /// False if there are any [`Self::duplicates`], [`Self::incomplete`]
    /// cells, or [`Self::invalid_values`].
    pub fn is_ok(&self) -> bool {
        self.duplicates.is_empty() && self.incomplete.is_empty() && self.invalid_values.is_empty()
    }
}

impl Board {
    /// Validate the board to ensure it is still solvable.
    ///
    /// This checks that no rows, columns, or regions contain duplicate values.
    /// If `solved` is true, then it also checks that all cells are filled.
    ///
    /// This function checks the whole board, even if it is partially filled.
    /// All problems are aggregated and returned in a [`ValidationResults`] struct.
    pub fn validate(&self, solved: bool) -> ValidationResults {
        let max_cells = self.size.max_cells() as usize;
        let mut results = ValidationResults::default();

        // Note: We don't need to check grid len() because of `Board` constructors
        for row in 0..max_cells {
            for col in 0..max_cells {
                let cell = &self.grid[row][col];
                let visiting = Coord { row, col };
                if let Cell::Value(value) = *cell {
                    if value >= max_cells as u8 {
                        results.push_invalid_value(visiting);
                        continue;
                    }

                    // Check row and column for duplicates
                    for i in 0..max_cells {
                        let test_cell = Coord { row, col: i };
                        if i != col {
                            match self.grid[row][i].is_valid(value, solved) {
                                ValidatedCell::Dupe => {
                                    let dupes = vec![visiting, test_cell];
                                    results.push_duplicate(value, dupes);
                                }
                                ValidatedCell::Incomplete => {
                                    results.push_incomplete(test_cell);
                                }
                                ValidatedCell::Ok => (),
                            }
                        }
                        if i != row {
                            let test_cell = Coord { row: i, col };
                            match self.grid[i][col].is_valid(value, solved) {
                                ValidatedCell::Dupe => {
                                    let dupes = vec![visiting, test_cell];
                                    results.push_duplicate(value, dupes);
                                }
                                ValidatedCell::Incomplete => {
                                    results.push_incomplete(test_cell);
                                }
                                ValidatedCell::Ok => (),
                            }
                        }
                    }

                    // Check region for duplicates
                    let siblings = self.kind.siblings(&self.size, Coord { row, col });
                    for coord in siblings {
                        match self.grid[coord.row][coord.col].is_valid(value, solved) {
                            ValidatedCell::Dupe => {
                                let dupes = vec![visiting, coord];
                                results.push_duplicate(value, dupes);
                            }
                            ValidatedCell::Incomplete => {
                                results.push_incomplete(coord);
                            }
                            ValidatedCell::Ok => (),
                        }
                    }
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Board, Cell, Coord};

    #[test]
    fn invalid_board() {
        // for this test we need a static and solved board (no randomness)
        let board = Board::from_str(include_str!("../tests/test-solution.txt")).unwrap();
        assert!(board.validate(true).is_ok());
        println!("--- Valid Board ---");
        println!("{board}");
        let faulty_value = board
            .get_cell(Coord::default())
            .unwrap()
            .get_value()
            .unwrap();

        let mut invalid_board = board.clone();
        invalid_board.grid[0][0] = Cell::Value(faulty_value);
        // Set a duplicate values in the same row
        invalid_board.grid[0][1] = Cell::Value(faulty_value);
        // Set a duplicate values in the same column
        invalid_board.grid[1][0] = Cell::Value(faulty_value);
        // Set some duplicate values in the same region
        invalid_board.grid[1][1] = Cell::Value(faulty_value);
        // Set a cell in first region to empty
        invalid_board.grid[2][2] = Cell::Notes(vec![]);
        // Set a cell in first row to empty
        invalid_board.grid[0][2] = Cell::Notes(vec![]);
        // Set a cell in first col to empty
        invalid_board.grid[2][0] = Cell::Notes(vec![]);
        // Set a cell in first col to a value out of regions' bounds
        invalid_board.grid[2][1] = Cell::Value(board.size.max_cells());

        println!("--- Invalid Board ---");
        println!("{invalid_board}");

        let mut result = invalid_board.validate(true);
        assert!(!result.is_ok());

        let duplicates = vec![
            Coord { row: 0, col: 0 },
            Coord { row: 0, col: 1 },
            Coord { row: 1, col: 0 },
            Coord { row: 1, col: 1 },
            Coord { row: 3, col: 1 }, // existing cells conflicting with faulty_value
            Coord { row: 1, col: 7 }, // existing cells conflicting with faulty_value
        ];
        assert_eq!(result.duplicates[&faulty_value], duplicates);

        assert_eq!(
            result.incomplete,
            vec![
                Coord { row: 0, col: 2 },
                Coord { row: 2, col: 0 },
                Coord { row: 2, col: 2 },
            ]
        );

        let invalid_coord = Coord { row: 2, col: 1 };
        assert_eq!(result.invalid_values, vec![invalid_coord]);
        result.push_invalid_value(invalid_coord); // should be non-op (for code coverage)
        assert_eq!(result.invalid_values, vec![invalid_coord]);
    }
}
