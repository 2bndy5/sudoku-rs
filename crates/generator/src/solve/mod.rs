use crate::{Board, Cell, Coord};
mod region_notes;
use region_notes::{LockedCandidate, RegionNotes};

impl Board {
    fn has_unsolvable_cells(&self) -> bool {
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

    fn get_naked_single(&self) -> Option<(Coord, u8)> {
        let max_cells = self.size.max_cells() as usize;
        for row in 0..max_cells {
            for col in 0..max_cells {
                let cell = &self.grid[row][col];
                if let Cell::Notes(notes) = &cell
                    && notes.len() == 1
                {
                    return Some((Coord { row, col }, notes[0]));
                }
            }
        }
        None
    }

    /// For best explanation of this function,
    /// see: https://sudoku.coach/en/learn/technique-overview
    fn set_hidden_singles_and_use_locked_candidates1(&mut self) {
        // Fortunately, each board is guaranteed to have the same number of
        // regions as the maximum number of cells in a row or column.
        for region in 0..self.size.max_cells() {
            let mut region_notes = RegionNotes::new();
            region_notes.scan(self, region);

            // set all hidden singles
            let hidden_singles = region_notes.find_singles();
            for (coord, value) in region_notes.find_singles() {
                self.set_cell(coord, value);
            }

            // remove hidden singles from region notes
            // This allows us to reuse the region notes to efficiently
            // find locked candidates (per region).
            let removed_candidates = hidden_singles.iter().map(|n| n.1).collect::<Vec<u8>>();
            region_notes
                .notes
                .retain(|n| !removed_candidates.contains(&n.candidate));

            // use locked candidates to remove candidates from other regions
            for locked_candidate in region_notes.find_locked_candidates() {
                match locked_candidate {
                    LockedCandidate::Row(candidate, row) => {
                        for (i, col) in self.grid[row].iter_mut().enumerate() {
                            if let Cell::Notes(notes) = col
                                && self
                                    .kind
                                    .index(&self.size, Coord { row, col: i })
                                    .is_some_and(|reg| reg != region)
                            {
                                notes.retain(|v| *v != candidate);
                            }
                        }
                    }
                    LockedCandidate::Column(candidate, col) => {
                        for (i, row) in self.grid.iter_mut().enumerate() {
                            if let Cell::Notes(notes) = &mut row[col]
                                && self
                                    .kind
                                    .index(&self.size, Coord { row: i, col })
                                    .is_some_and(|reg| reg != region)
                            {
                                notes.retain(|v| *v != candidate);
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_cell_with_least_notes(&self) -> Option<Coord> {
        let max_cells = self.size.max_cells() as usize;
        let mut min_notes = max_cells;
        let mut target_cell = None;
        for row in 0..max_cells {
            for col in 0..max_cells {
                if let Cell::Notes(notes) = &self.grid[row][col] {
                    let len = notes.len();
                    if len < min_notes && len > 0 {
                        min_notes = len;
                        target_cell = Some(Coord { row, col });
                    }
                }
            }
        }
        target_cell
    }

    /// Solve the board using a backtracking algorithm (recursive).
    ///
    /// Returns `true` if the board was solved, or `false` if it is unsolvable.
    pub fn solve(&mut self) -> bool {
        // first set all cells with only 1 note remaining
        while let Some((coord, value)) = self.get_naked_single() {
            self.set_cell(coord, value);
        }

        if self.has_unsolvable_cells() {
            return false;
        }

        self.set_hidden_singles_and_use_locked_candidates1();

        // next find cell with the least notes remaining
        if let Some(coord) = self.get_cell_with_least_notes()
            && let Cell::Notes(notes) = &self.grid[coord.row][coord.col]
        {
            for note in notes {
                let mut new_board = Box::new(self.clone());
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

    use crate::{Board, BoardSize};

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
        let mut board = Board::new(&size, crate::BoardKind::Regular);
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
