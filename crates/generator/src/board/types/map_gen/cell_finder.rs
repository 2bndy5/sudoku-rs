//! a private module for finding neighboring cells in a board.
use crate::cell::Coord;

#[derive(Debug)]
pub enum CellFinder {
    North,
    East,
    South,
    West,
}

impl CellFinder {
    /// Increment the given row and column based on the direction.
    ///
    /// Returns None if the increment would go out of bounds.
    fn inc(&self, coord: Coord, max_index: usize) -> Option<Coord> {
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

    pub fn list() -> Vec<Self> {
        vec![Self::North, Self::East, Self::South, Self::West]
    }

    /// Get any perpendicular neighboring cell coordinates.
    pub fn get_neighbors(max_index: usize, coord: Coord) -> Vec<Coord> {
        let mut neighbors = vec![];
        for direction in Self::list() {
            if let Some(neighbor) = direction.inc(coord, max_index) {
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}
