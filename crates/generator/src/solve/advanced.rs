use crate::{
    Board, Cell, Coord, LineKind,
    region::{LockedCandidate, RegionNotes},
};

impl Board {
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
    pub(super) fn use_region_candidates(&mut self) -> bool {
        // Fortunately, each board is guaranteed to have the same number of
        // regions as the maximum number of cells in a row or column.
        let mut any_changed = false;
        for region in 0..self.size.max_cells() {
            let mut region_notes = RegionNotes::new();
            region_notes.scan(self, region);

            // set all hidden singles
            let hidden_singles = region_notes.find_singles();
            for (coord, value) in region_notes.find_singles() {
                any_changed = true;
                self.set_cell(coord, value);
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
                        for (i, col) in self.grid[*row].iter_mut().enumerate() {
                            if let Cell::Notes(notes) = col
                                && self
                                    .kind
                                    .index(&self.size, Coord { row: *row, col: i })
                                    .is_some_and(|reg| reg != region)
                                && notes.contains(candidate)
                            {
                                any_changed = true;
                                notes.retain(|v| *v != *candidate);
                            }
                        }
                    }
                    LockedCandidate::Column(candidate, col) => {
                        for (i, row) in self.grid.iter_mut().enumerate() {
                            if let Cell::Notes(notes) = &mut row[*col]
                                && self
                                    .kind
                                    .index(&self.size, Coord { row: i, col: *col })
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

    /// Eliminate candidates using locked candidates technique (type 2).
    ///
    /// This technique is like an inversion of the locked candidates type 1 technique.
    /// Also known as "hidden pairs/triples/quads".
    /// We look for common candidates shared among cells in the same row or column,
    /// and eliminates the candidate(s) that are not shared among the cells.
    ///
    /// This is done separately from `use_hidden_singles_and_locked_candidates1()`
    /// because it requires a board-wide scope. Whereas, locked candidates type 1
    /// can be done within the scope of a single region.
    pub(super) fn expose_hidden_sets(&mut self) -> bool {
        let max_cells = self.size.max_cells() as usize;
        let mut any_changed = false;
        for i in 0..max_cells {
            for line_kind in [LineKind::Row(i), LineKind::Column(i)] {
                let shared_candidates = line_kind.find_cells_with_shared_candidates(self);
                // eliminate non-shared candidates from cells that share candidates
                for (candidates, coords) in shared_candidates {
                    for coord in coords {
                        if let Cell::Notes(notes) = &mut self.grid[coord.row][coord.col]
                            && notes.len() != candidates.len()
                        {
                            // println!("notes for {coord}: {notes:?} became {candidates:?}");
                            any_changed = true;
                            notes.clone_from(&candidates);
                        }
                    }
                }
            }
        }
        any_changed
    }
}
