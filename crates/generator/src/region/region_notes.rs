use crate::{
    Board,
    cell::{Cell, Coord},
};

pub struct Candidate {
    cells: Vec<Coord>,
    pub value: u8,
}

pub struct RegionNotes {
    pub notes: Vec<Candidate>,
}

/// A returned type to differentiate between Row or Column locked candidates.
pub enum LockedCandidate {
    Row(u8, usize),
    Column(u8, usize),
}

impl RegionNotes {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    pub fn scan(&mut self, board: &Board, index: u8) {
        let members = board.kind.members(&board.size, index);
        for coord in members {
            if let Cell::Notes(notes) = &board.grid[coord.row][coord.col] {
                for note in notes {
                    self.push_notes(coord, *note);
                }
            }
        }
    }

    fn push_notes(&mut self, coord: Coord, candidate: u8) {
        if let Some(noted_cells) = self.notes.iter_mut().find(|n| n.value == candidate) {
            noted_cells.cells.push(coord);
        } else {
            self.notes.push(Candidate {
                cells: vec![coord],
                value: candidate,
            });
        }
    }

    pub fn find_singles(&self) -> Vec<(Coord, u8)> {
        let mut singles = Vec::new();
        for possible in &self.notes {
            if possible.cells.len() == 1 {
                let coord = possible.cells[0];
                singles.push((coord, possible.value));
            }
        }
        singles
    }

    pub fn find_locked_candidates(&self) -> Vec<LockedCandidate> {
        let mut pairs = Vec::new();
        // If candidates are sequestered to the same row or column,
        // then we have a pointed pair (AKA "locked candidate").
        for possible in &self.notes {
            let mut iter = possible.cells.iter();
            // safe to unwrap() the first element because
            // the candidate wouldn't be allocated if there are no cells.
            let (mut common_row, mut common_col) = iter
                .next()
                .map(|coord| (Some(coord.row), Some(coord.col)))
                .unwrap();
            for coord in iter {
                if common_row.is_some_and(|v| v != coord.row) {
                    common_row = None;
                }
                if common_col.is_some_and(|v| v != coord.col) {
                    common_col = None;
                }
            }
            if let Some(r) = common_row {
                pairs.push(LockedCandidate::Row(possible.value, r));
            }
            if let Some(c) = common_col {
                pairs.push(LockedCandidate::Column(possible.value, c));
            }
        }
        // let the caller remove notes in other regions that are "pointed" by these pairs.
        pairs
    }
}
