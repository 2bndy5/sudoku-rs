use std::str::FromStr;

#[cfg(feature = "clap")]
use clap::ValueEnum;
use rand::seq::SliceRandom;

use crate::{
    Board, Coord, Strategy,
    solve::{HiddenSets, HiddenSingles, NakedSingles, RegionLineReduction},
};

/// The difficulty levels for a sudoku puzzle.
///
/// These levels are used to determine which solving strategies
/// must be applied to solve a puzzle.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Difficulty {
    fn assess(&self, board: &mut Board) -> bool {
        let mut changed = false;
        if NakedSingles::DIFFICULTY_LEVEL <= *self && NakedSingles::apply(board) {
            changed = true;
        }
        if HiddenSingles::DIFFICULTY_LEVEL <= *self && HiddenSingles::apply(board) {
            changed = true;
        }
        if RegionLineReduction::DIFFICULTY_LEVEL <= *self && RegionLineReduction::apply(board) {
            changed = true;
        }
        if HiddenSets::DIFFICULTY_LEVEL <= *self && HiddenSets::apply(board) {
            changed = true;
        }
        changed
    }

    /// Generate a puzzle from a solved board.
    ///
    /// The puzzle returned will be a clone of the given solution board,
    /// with cells cleared according to the difficulty level.
    pub fn make_puzzle(&self, solution: &Board) -> Board {
        let mut puzzle = solution.clone();
        let max_cells = solution.size.max_cells() as usize;
        let total_cells = max_cells.pow(2);
        let min_clues = (total_cells as f32 * 0.21) as usize;
        let mut all_cells = Vec::from_iter(0..total_cells);
        let mut rng = rand::rng();
        let mut changed = true;
        let mut count = 0;
        while changed {
            count += 1;
            changed = false;
            all_cells.shuffle(&mut rng);
            for cell_index in all_cells.clone() {
                let coord = Coord {
                    row: cell_index / max_cells,
                    col: cell_index % max_cells,
                };
                let mut temp_board = puzzle.clone();
                temp_board.clear_cell(coord);
                while self.assess(&mut temp_board) {}
                if temp_board.get_cell(coord).is_some_and(|v| v.is_value())
                    && all_cells.len() > min_clues
                {
                    all_cells.retain(|v| *v != cell_index);
                    changed = true;
                    puzzle.clear_cell(coord);
                }
            }
        }
        println!(
            "iterated {count} times to remove {} of {total_cells} cells ({} clues)",
            total_cells - all_cells.len(),
            all_cells.len()
        );
        puzzle
    }

    pub fn as_str(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            "expert" => Ok(Difficulty::Expert),
            _ => Ok(Difficulty::Easy),
        }
    }
}
