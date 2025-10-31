mod difficulty;
mod strategy;
use crate::{Board, Cell, Coord};
pub use difficulty::Difficulty;
pub use strategy::{HiddenSets, HiddenSingles, NakedSingles, RegionLineReduction, Strategy};

impl Board {
    /// Does the board have any unfilled cells with empty notes?
    ///
    /// This indicates that the board is unsolvable.
    /// Internally, this is used to check if a solving [`Strategy`]
    /// has made the board unsolvable.
    pub fn has_unsolvable_cells(&self) -> bool {
        let max_cells = self.size.max_cells();
        for row in 0..max_cells as usize {
            for col in 0..max_cells as usize {
                if let Cell::Notes(notes) = &self.grid[row][col]
                    && notes.is_empty()
                {
                    return true;
                }
            }
        }
        false
    }

    /// Get the coordinate of the cell with the least number of notes remaining.
    ///
    /// This function serves as starting point for guessing in the backtracking algorithm.
    ///
    /// Returns `None` if there are no cells with notes remain.
    pub fn get_cell_with_least_notes(&self) -> Option<Coord> {
        let max_cells = self.size.max_cells() as usize;
        let mut min_notes = max_cells;
        let mut target_cell = None;
        for row in 0..max_cells {
            if min_notes == 1 {
                break; // can't do better than this
            }
            for col in 0..max_cells {
                if let Cell::Notes(notes) = &self.grid[row][col] {
                    let len = notes.len();
                    if len < min_notes && len > 0 {
                        min_notes = len;
                        target_cell = Some(Coord { row, col });
                    }
                }
                if min_notes == 1 {
                    break; // can't do better than this
                }
            }
        }
        target_cell
    }

    /// Solve the board using a backtracking algorithm (recursive).
    ///
    /// Returns `true` if the board was solved, or `false` if it is unsolvable.
    pub fn solve(&mut self) -> bool {
        if self.has_unsolvable_cells() {
            return false;
        }

        while RegionLineReduction::apply(self) {
            if self.has_unsolvable_cells() {
                return false;
            }
        }

        while HiddenSingles::apply(self) {
            if self.has_unsolvable_cells() {
                return false;
            }
        }

        if NakedSingles::apply(self) && self.has_unsolvable_cells() {
            return false;
        }

        if HiddenSets::apply(self) {
            // don't guess yet, restart with the new notes
            return self.solve();
        }

        // next find cell with the least notes remaining
        if let Some(coord) = self.get_cell_with_least_notes()
            && let Cell::Notes(notes) = &self.grid[coord.row][coord.col]
        {
            for note in notes {
                let mut new_board = Box::new(self.clone());
                println!("guessing {note} for {coord}");
                new_board.set_cell(coord, *note);
                if new_board.solve() {
                    *self = *new_board;
                    return true;
                }
            }
            return false;
        }
        // if no cells with notes remain, then we are done
        true
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Board, BoardSize, RegionKind};

    #[test]
    fn import_solve() {
        let board_str = include_str!("../../tests/basic.txt");
        let mut board = Board::from_str(board_str).unwrap();
        println!("{board}");
        assert!(board.validate(false).is_ok());
        assert!(board.solve());
        println!("{board}");
        assert!(board.validate(true).is_ok());
    }

    fn gen_solve(size: BoardSize) {
        let mut board = Board::new(&size, RegionKind::Regular);
        assert!(board.generate());
        println!("Solved:\n{board}");
        assert!(board.validate(true).is_ok());
    }

    #[test]
    fn gen_solve_4x4() {
        gen_solve(BoardSize::X4);
    }

    #[test]
    fn gen_solve_6x6() {
        gen_solve(BoardSize::X6);
    }

    #[test]
    fn gen_solve_9x9() {
        gen_solve(BoardSize::X9);
    }

    #[test]
    fn gen_solve_12x12() {
        gen_solve(BoardSize::X12);
    }

    #[test]
    fn gen_solve_16x16() {
        gen_solve(BoardSize::X16);
    }
}
