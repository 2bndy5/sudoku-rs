use super::Difficulty;
use crate::Board;
mod naked_singles;
pub use naked_singles::NakedSingles;
mod hidden_singles;
pub use hidden_singles::HiddenSingles;
mod hidden_sets;
pub use hidden_sets::HiddenSets;
mod region_line_reduction;
pub use region_line_reduction::RegionLineReduction;

/// A generic trait to describe a strategy for solving a Sudoku puzzle.
pub trait Strategy {
    const DIFFICULTY_LEVEL: Difficulty;

    fn apply(board: &mut Board) -> bool;
}
