mod advanced;
mod basic;
use crate::{Board, Cell};

impl Board {
    /// Solve the board using a backtracking algorithm (recursive).
    ///
    /// Returns `true` if the board was solved, or `false` if it is unsolvable.
    pub fn solve(&mut self) -> bool {
        if self.has_unsolvable_cells() {
            return false;
        }

        while self.use_region_candidates() {
            if self.has_unsolvable_cells() {
                return false;
            }
        }

        while self.set_hidden_singles() {
            if self.has_unsolvable_cells() {
                return false;
            }
        }

        while let Some((coord, value)) = self.get_naked_single() {
            self.set_cell(coord, value);
            if self.has_unsolvable_cells() {
                return false;
            }
        }

        if self.expose_hidden_sets() {
            // don't guess yet, restart with the new notes
            return self.solve();
        }

        // next find cell with the least notes remaining
        if let Some(coord) = self.get_cell_with_least_notes()
            && let Cell::Notes(notes) = &self.grid[coord.row][coord.col]
        {
            for note in notes {
                let mut new_board = Box::new(self.clone());
                // println!("guessing {note} for {coord}");
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
