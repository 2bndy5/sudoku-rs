use rand::seq::SliceRandom;

use crate::{Board, cell::Coord};

impl Board {
    /// Seed the board with some random valid values.
    ///
    /// This populates the board's first region (containing the
    /// first cell) with random, non-repeating values
    /// ranging from `0` (inclusive) up to
    /// [`BoardSize::max_cells()`](fn@crate::BoardSize::max_cells) (exclusive).
    pub fn seed(&mut self) {
        let max_cells = self.size.max_cells() as usize;
        let mut cells = Vec::from_iter(0..max_cells as u8);
        let mut rng = rand::rng();
        cells.shuffle(&mut rng);
        self.set_cell_unchecked_grid(Coord::default(), cells[0]);
        for (index, coord) in self
            .kind
            .siblings(&self.size, Coord::default())
            .iter()
            .enumerate()
        {
            self.set_cell_unchecked_grid(*coord, cells[index + 1]);
        }
    }

    /// Generate a completed Sudoku puzzle.
    ///
    /// This is shorthand for calling [`Board::seed()`] followed by [`Board::solve()`].
    pub fn generate(&mut self) -> bool {
        self.seed();
        self.solve()
    }
}

#[cfg(test)]
mod test {
    use crate::{BoardSize, IrregularMap, RegionKind, cell::Coord};

    use super::Board;

    fn seed_size(size: BoardSize) {
        let row_width = size.width();
        let col_height = size.height();
        let mut board = Board::new(&size, RegionKind::Regular);
        board.seed();
        println!("{board}");
        for i in 0..board.size.max_cells() as usize {
            let col = i % col_height as usize;
            let row = i / row_width as usize;
            assert!(board.grid[row][col].is_value());
        }
    }

    #[test]
    fn seed_small() {
        seed_size(BoardSize::X4);
    }

    #[test]
    fn seed_rect6() {
        seed_size(BoardSize::X6);
    }

    #[test]
    fn seed_standard() {
        seed_size(BoardSize::X9);
    }

    #[test]
    fn seed_rect12() {
        seed_size(BoardSize::X12);
    }

    #[test]
    fn seed_large() {
        seed_size(BoardSize::X16);
    }

    #[test]
    fn seed_irregular() {
        let mut board = Board::new(
            &BoardSize::X4,
            RegionKind::Irregular {
                map: IrregularMap::new(vec![
                    vec![0, 0, 0, 1],
                    vec![0, 3, 1, 1],
                    vec![3, 3, 1, 2],
                    vec![3, 2, 2, 2],
                ])
                .unwrap(),
            },
        );
        board.seed();
        assert!(board.validate(false).is_ok());
        println!("--- Seeded ---");
        println!("{board}");
        // Due to the irregular grid map above, we know that
        // the first region contains the first cell (0, 0).
        assert!(board.grid[0][0].is_value());
        for coord in board.kind.siblings(&board.size, Coord::default()) {
            assert!(board.grid[coord.row][coord.col].is_value());
        }
        assert!(board.solve());
        println!("--- Solved ---");
        println!("{board}");
        assert!(board.validate(true).is_ok());
    }
}
