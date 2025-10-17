use std::{collections::HashSet, fmt::Display};
mod finder;
pub use finder::{Cardinal, CoordWalker};

pub(crate) enum ValidatedCell {
    Dupe,
    Incomplete,
    Ok,
}

/// A cell in a Sudoku board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    /// The value of an filled cell.
    Value(u8),

    /// The notes for an empty cell.
    ///
    /// These are the possible candidates that could go in the [`Cell::Value`].
    Notes(HashSet<u8>),
}

impl Cell {
    /// A reusable function for validating a [`Cell`].
    ///
    /// If the cell is a [`Cell::Value`], then it checks if the value is not equal to `dupe`.
    ///
    /// If the cell is a [`Cell::Notes`], then it checks if `solved` is `false`.
    ///
    /// This is used repeatedly in [`Board::validate()`](fn@crate::Board::validate)
    /// to check for duplicates in rows, columns, and regions.
    /// Setting `solved` to `false` allows validating an incomplete solution.
    pub(crate) const fn is_valid(&self, dupe: u8, solved: bool) -> ValidatedCell {
        match self {
            Cell::Value(v) => {
                if *v == dupe {
                    ValidatedCell::Dupe
                } else {
                    ValidatedCell::Ok
                }
            }
            Cell::Notes(_) => {
                if solved {
                    ValidatedCell::Incomplete
                } else {
                    ValidatedCell::Ok
                }
            }
        }
    }

    /// Transforms a [`Cell::Notes`] (by reference) into an [`Option`]
    pub fn get_notes(&self) -> Option<&HashSet<u8>> {
        match self {
            Cell::Notes(notes) => Some(notes),
            Cell::Value(_) => None,
        }
    }

    /// Transforms a [`Cell::Value`] into an [`Option`]
    pub fn get_value(&self) -> Option<u8> {
        match self {
            Cell::Value(v) => Some(*v),
            Cell::Notes(_) => None,
        }
    }

    /// [`Option::is_some()`] for [`Cell::Value`]
    pub const fn is_value(&self) -> bool {
        match self {
            Cell::Value(_) => true,
            Cell::Notes(_) => false,
        }
    }

    /// [`Option::is_some()`] for [`Cell::Notes`]
    pub const fn is_notes(&self) -> bool {
        match self {
            Cell::Notes(_) => true,
            Cell::Value(_) => false,
        }
    }
}

/// A coordinate on a Sudoku board.
///
/// Both `row` and `col` use a zero-based index.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    /// The row of the coordinate (0 based count).
    pub row: usize,

    /// The column of the coordinate (0 based count).
    pub col: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::Cell;
    #[test]
    fn no_notes() {
        let cell = Cell::Value(5);
        assert!(cell.get_notes().is_none());
        assert!(cell.get_value().is_some());
    }

    #[test]
    fn is_none() {
        let cell = Cell::Notes(HashSet::new());
        assert!(cell.get_value().is_none());
        assert!(cell.get_notes().is_some());
    }

    #[test]
    fn is_value() {
        let cell = Cell::Value(5);
        assert!(cell.is_value());
        assert!(!cell.is_notes());
    }

    #[test]
    fn is_notes() {
        let cell = Cell::Notes(HashSet::from_iter([1, 2, 3]));
        assert!(cell.is_notes());
        assert!(!cell.is_value());
    }

    #[test]
    fn display_coord() {
        let coord = super::Coord { row: 3, col: 5 };
        assert_eq!(format!("{}", coord), "(3, 5)");
    }
}
