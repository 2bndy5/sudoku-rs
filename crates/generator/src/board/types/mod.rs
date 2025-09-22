mod map_gen;
pub use map_gen::{IrregularMap, IrregularMapError};

use crate::Coord;

/// The size of the Sudoku board.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BoardSize {
    /// A 4x4 Sudoku board.
    X4,

    /// A 6x6 Sudoku board.
    X6,

    /// A 9x9 Sudoku board.
    #[default]
    X9,

    /// A 12x12 Sudoku board.
    X12,

    /// A 16x16 Sudoku board.
    X16,
}

impl From<u8> for BoardSize {
    fn from(size: u8) -> Self {
        match size {
            4 => BoardSize::X4,
            6 => BoardSize::X6,
            12 => BoardSize::X12,
            16 => BoardSize::X16,
            _ => BoardSize::X9,
        }
    }
}

impl BoardSize {
    /// Get the maximum number of cells in any row or column.
    pub const fn max_cells(&self) -> u8 {
        match self {
            BoardSize::X4 => 4,
            BoardSize::X6 => 6,
            BoardSize::X9 => 9,
            BoardSize::X12 => 12,
            BoardSize::X16 => 16,
        }
    }

    /// Get the width of each region in the board.
    ///
    /// This is only applicable to [`BoardKind::Regular`] boards.
    pub const fn width(&self) -> u8 {
        match self {
            BoardSize::X4 => 2,
            BoardSize::X6 => 3,
            BoardSize::X9 => 3,
            BoardSize::X12 => 4,
            BoardSize::X16 => 4,
        }
    }

    /// Get the height of each region in the board.
    ///
    /// This is only applicable to [`BoardKind::Regular`] boards.
    pub const fn height(&self) -> u8 {
        match self {
            BoardSize::X4 => 2,
            BoardSize::X6 => 2,
            BoardSize::X9 => 3,
            BoardSize::X12 => 3,
            BoardSize::X16 => 4,
        }
    }
}

/// The type of grids used in the board.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BoardKind {
    /// A standard rectangular grid of regions.
    #[default]
    Regular,

    /// An irregular (AKA "Jigsaw") grid of regions.
    ///
    /// This type does not guarantees that all regions are not rectangular.
    /// Some regions may be rectangular, while others are not.
    /// Smaller boards (eg. [`4x4`](type@crate::BoardSize::X4) and [`6x6`](type@crate::BoardSize::X6))
    /// are more likely to have rectangular regions due to the limited range of permutations.
    Irregular {
        /// A mapping of the cells to their respective region's index.
        map: IrregularMap,
    },
}

impl BoardKind {
    /// Get the index of the region for the given `coord`.
    ///
    /// If the `coord` is out of bounds (per [`BoardSize::max_cells()`]),
    /// then [`None`] is returned.
    pub fn index(&self, size: &BoardSize, coord: Coord) -> Option<u8> {
        let max_cells = size.max_cells() as usize;
        if coord.row < max_cells && coord.col < max_cells {
            match self {
                BoardKind::Regular => {
                    let width = size.width();
                    let height = size.height();
                    Some(((coord.row as u8) / height * height) + ((coord.col as u8) / width))
                }
                BoardKind::Irregular { map } => {
                    // map is guaranteed to be the correct size per IrregularMap constructors
                    Some(map.regions[coord.row][coord.col])
                }
            }
        } else {
            None
        }
    }

    /// Get all the sibling cells for a region containing the given `coord`.
    ///
    /// Similar to [`Self::members()`] but excludes the given `coord` for
    /// slightly better performance.
    pub fn siblings(&self, size: &BoardSize, coord: Coord) -> Vec<Coord> {
        let max_cells = size.max_cells() as usize;
        let mut siblings = Vec::new();
        if let Some(index) = self.index(size, coord) {
            let mut found_all = false;
            for row in 0..max_cells {
                if found_all {
                    break;
                }
                for col in 0..max_cells {
                    let sibling = Coord { row, col };
                    if sibling == coord {
                        continue;
                    }
                    if let Some(i) = self.index(size, sibling)
                        && i == index
                    {
                        siblings.push(sibling);
                    }
                    if siblings.len() >= max_cells - 1 {
                        found_all = true;
                        // short circuit the search if we're already done
                        break;
                    }
                }
            }
        }
        siblings
    }

    /// Get all the members of a region for the given index.
    pub fn members(&self, size: &BoardSize, index: u8) -> Vec<Coord> {
        let max_cells = size.max_cells() as usize;
        let mut members = Vec::new();
        let mut found_all = false;
        for row in 0..max_cells {
            if found_all {
                break;
            }
            for col in 0..max_cells {
                let member_coord = Coord { row, col };
                if let Some(i) = self.index(size, member_coord)
                    && i == index
                {
                    members.push(member_coord);
                }
                if members.len() >= max_cells {
                    found_all = true;
                    // short circuit the search if we're already done
                    break;
                }
            }
        }
        members
    }
}

#[cfg(test)]
mod test {
    use crate::{BoardKind, BoardSize, Coord, IrregularMap};

    #[test]
    fn grid_index_valid() {
        let size = BoardSize::X9;
        let kind = BoardKind::Regular;
        let index = kind.index(&size, Coord { row: 5, col: 7 });
        assert_eq!(index, Some(5));

        let size = BoardSize::X4;
        let kind = BoardKind::Irregular {
            map: IrregularMap::new(vec![
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1],
                vec![2, 2, 3, 3],
                vec![2, 2, 3, 3],
            ])
            .unwrap(),
        };
        let index = kind.index(&size, Coord { row: 2, col: 3 });
        assert_eq!(index, Some(3));
    }

    #[test]
    fn grid_index_invalid() {
        let size = BoardSize::X9;
        let kind = BoardKind::Regular;
        let index = kind.index(&size, Coord { row: 9, col: 8 });
        assert_eq!(index, None);
        let index = kind.index(&size, Coord { row: 8, col: 9 });
        assert_eq!(index, None);

        let size = BoardSize::X4;
        let kind = BoardKind::Irregular {
            map: IrregularMap::generate(&size).unwrap(),
        };
        let max_cells = size.max_cells() as usize;
        let index = kind.index(
            &size,
            Coord {
                row: max_cells,
                col: 0,
            },
        );
        assert_eq!(index, None);
        let index = kind.index(
            &size,
            Coord {
                row: 0,
                col: max_cells,
            },
        );
        assert_eq!(index, None);
    }

    #[test]
    fn grid_siblings() {
        let size = BoardSize::X9;
        let kind = BoardKind::Regular;
        let siblings = kind.siblings(&size, Coord { row: 5, col: 7 });
        assert_eq!(siblings.len(), 8);
        assert!(siblings.contains(&Coord { row: 5, col: 6 }));
        assert!(siblings.contains(&Coord { row: 4, col: 7 }));
        assert!(siblings.contains(&Coord { row: 3, col: 6 }));
    }
}
