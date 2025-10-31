use super::Strategy;
use crate::{Board, Cell, Coord, Difficulty};

pub struct NakedSingles;

impl NakedSingles {
    fn get_naked_single(board: &Board) -> Option<(Coord, u8)> {
        let max_cells = board.size.max_cells() as usize;
        for row in 0..max_cells {
            for col in 0..max_cells {
                if let Cell::Notes(notes) = &board.grid[row][col]
                    && notes.len() == 1
                {
                    // safe to unwrap since we checked len() == 1
                    return Some((Coord { row, col }, notes.iter().next().copied().unwrap()));
                }
            }
        }
        None
    }
}

impl Strategy for NakedSingles {
    const DIFFICULTY_LEVEL: Difficulty = Difficulty::Easy;

    fn apply(board: &mut Board) -> bool {
        let mut changed = false;
        while let Some((coord, value)) = Self::get_naked_single(board) {
            board.set_cell(coord, value);
            changed = true;
        }
        changed
    }
}
