use crate::{Board, Cell, Coord, lines::LineKind};

impl Board {
    pub(super) fn has_unsolvable_cells(&self) -> bool {
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

    pub(super) fn get_naked_single(&self) -> Option<(Coord, u8)> {
        let max_cells = self.size.max_cells() as usize;
        for row in 0..max_cells {
            for col in 0..max_cells {
                if let Cell::Notes(notes) = &self.grid[row][col]
                    && notes.len() == 1
                {
                    // safe to unwrap since we checked len() == 1
                    return Some((Coord { row, col }, notes.iter().next().copied().unwrap()));
                }
            }
        }
        None
    }

    pub(super) fn set_hidden_singles(&mut self) -> bool {
        let mut any_changed = false;
        let max_cells = self.size.max_cells() as usize;
        for i in 0..max_cells {
            for line in [LineKind::Row(i), LineKind::Column(i)] {
                let candidates = line.map_notes_to_coords(self);
                for (candidate, coords) in candidates {
                    let mut it = coords.iter();
                    if let Some(coord) = it.next()
                        && it.len() == 0
                    {
                        self.set_cell(*coord, candidate);
                        any_changed = true;
                    }
                }
            }
        }
        any_changed
    }

    pub(super) fn get_cell_with_least_notes(&self) -> Option<Coord> {
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
}
