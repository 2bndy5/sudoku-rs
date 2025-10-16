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
    /// This is only applicable to
    /// [`RegionKind::Regular`](crate::RegionKind::Regular)
    /// boards.
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
    /// This is only applicable to
    /// [`RegionKind::Regular`](crate::RegionKind::Regular)
    /// boards.
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
