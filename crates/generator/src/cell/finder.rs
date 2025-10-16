use super::Coord;

/// An enum of cardinal directions to find neighboring cells.
#[derive(Debug)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
    /// Increment the given `coord` in the [`Cardinal`]'s direction.
    ///
    /// Returns None if the increment would go out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use sudoku_gen::{Coord, Cardinal};
    /// let coord = Coord { row: 1, col: 1 };
    /// let max_index = 8; // for a standard 9x9 Sudoku board
    /// let north = Cardinal::North.next(coord, max_index);
    /// assert_eq!(north, Some(Coord { row: 0, col: 1 }));
    /// ```
    pub fn next(&self, coord: Coord, max_index: usize) -> Option<Coord> {
        match self {
            Self::North if coord.row > 0 => Some(Coord {
                row: coord.row - 1,
                col: coord.col,
            }),
            Self::East if coord.col < max_index => Some(Coord {
                row: coord.row,
                col: coord.col + 1,
            }),
            Self::South if coord.row < max_index => Some(Coord {
                row: coord.row + 1,
                col: coord.col,
            }),
            Self::West if coord.col > 0 => Some(Coord {
                row: coord.row,
                col: coord.col - 1,
            }),
            _ => None,
        }
    }

    /// List all supported directions.
    fn list() -> Vec<Self> {
        vec![Self::North, Self::East, Self::South, Self::West]
    }

    /// Get any perpendicular neighboring cell coordinates.
    ///
    /// # Example
    /// ```
    /// use sudoku_gen::{Coord, Cardinal};
    /// let coord = Coord { row: 1, col: 1 };
    /// let max_index = 8; // for a standard 9x9 Sudoku board
    /// let neighbors = Cardinal::get_neighbors(max_index, coord);
    /// assert_eq!(neighbors, vec![
    ///     Coord { row: 0, col: 1 }, // North
    ///     Coord { row: 1, col: 2 }, // East
    ///     Coord { row: 2, col: 1 }, // South
    ///     Coord { row: 1, col: 0 }, // West
    /// ]);
    /// ```
    pub fn get_neighbors(max_index: usize, coord: Coord) -> Vec<Coord> {
        let mut neighbors = vec![];
        for direction in Self::list() {
            if let Some(neighbor) = direction.next(coord, max_index) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}

/// An iterator that walks in a given direction from a starting coordinate.
pub struct CoordWalker {
    /// The starting coordinate.
    ///
    /// This coordinate is not included in the iterations.
    pub origin: Coord,

    /// The direction to walk.
    pub direction: Cardinal,

    /// The maximum index for rows and columns.
    ///
    /// This is typically [`BoardSize::max_cells()`](crate::BoardSize::max_cells()) - 1.
    pub max_index: usize,
}

impl Iterator for CoordWalker {
    type Item = Coord;

    /// Get the next coordinate in the [`CoordWalker::direction`].
    ///
    /// # Example
    /// ```
    /// use sudoku_gen::{Coord, CoordWalker, Cardinal};
    /// let start = Coord { row: 4, col: 4 };
    /// let max_index = 8; // for a standard 9x9 Sudoku board
    /// let walker = CoordWalker {
    ///     origin: start,
    ///     direction: Cardinal::South,
    ///     max_index,
    /// };
    /// let path: Vec<Coord> = walker.collect();
    /// assert_eq!(path, vec![
    ///     Coord { row: 5, col: 4 },
    ///     Coord { row: 6, col: 4 },
    ///     Coord { row: 7, col: 4 },
    ///     Coord { row: 8, col: 4 },
    /// ]);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_coord) = self.direction.next(self.origin, self.max_index) {
            self.origin = next_coord;
            Some(next_coord)
        } else {
            None
        }
    }
}
