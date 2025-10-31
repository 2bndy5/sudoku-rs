use crate::{
    Board, Cell, Coord, Difficulty, Strategy,
    region::{LockedCandidate, RegionNotes},
};

pub struct RegionLineReduction;

impl Strategy for RegionLineReduction {
    const DIFFICULTY_LEVEL: Difficulty = Difficulty::Medium;

    /// With a focus on individual regions, this function:
    ///
    /// 1. Identifies and sets cells that can only be a single candidate
    ///    (which is hidden among other candidates).
    /// 2. Applies the locked candidates technique (type 1) to eliminate candidates
    ///    from cells outside the region but within the same row or column.
    ///    This is also called pointed pairs/triples/quads because the cells
    ///    whose common candidates are confined to a single row or column in a region
    ///    form a pointing direction to other cells in the same row or column where
    ///    the shared candidate can be eliminated.
    ///
    /// Return `true` if any cells were set.
    /// Some candidates may still have been eliminated when this returns false.
    ///
    /// For best explanation of this function,
    /// see: https://sudoku.coach/en/learn/technique-overview
    fn apply(board: &mut Board) -> bool {
        // Fortunately, each board is guaranteed to have the same number of
        // regions as the maximum number of cells in a row or column.
        let mut any_changed = false;
        for region in 0..board.size.max_cells() {
            let mut region_notes = RegionNotes::new();
            region_notes.scan(board, region);

            // set all hidden singles
            let hidden_singles = region_notes.find_singles();
            for (coord, value) in region_notes.find_singles() {
                any_changed = true;
                board.set_cell(coord, value);
            }

            // remove hidden singles from region notes
            // This allows us to reuse the region notes to efficiently
            // find locked candidates (per region).
            let removed_candidates = hidden_singles.iter().map(|n| n.1).collect::<Vec<u8>>();
            region_notes
                .notes
                .retain(|n| !removed_candidates.contains(&n.value));

            // use locked candidates to remove candidates from other regions
            let locked_candidates = region_notes.find_locked_candidates();
            for locked_candidate in &locked_candidates {
                match locked_candidate {
                    LockedCandidate::Row(candidate, row) => {
                        for (i, col) in board.grid[*row].iter_mut().enumerate() {
                            if let Cell::Notes(notes) = col
                                && board
                                    .kind
                                    .index(&board.size, Coord { row: *row, col: i })
                                    .is_some_and(|reg| reg != region)
                                && notes.contains(candidate)
                            {
                                any_changed = true;
                                notes.retain(|v| *v != *candidate);
                            }
                        }
                    }
                    LockedCandidate::Column(candidate, col) => {
                        for (i, row) in board.grid.iter_mut().enumerate() {
                            if let Cell::Notes(notes) = &mut row[*col]
                                && board
                                    .kind
                                    .index(&board.size, Coord { row: i, col: *col })
                                    .is_some_and(|reg| reg != region)
                                && notes.contains(candidate)
                            {
                                any_changed = true;
                                notes.retain(|v| *v != *candidate);
                            }
                        }
                    }
                }
            }
        }
        any_changed
    }
}
