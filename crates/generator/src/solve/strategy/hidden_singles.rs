use super::Strategy;
use crate::{Board, Difficulty, LineKind};

pub struct HiddenSingles;

impl Strategy for HiddenSingles {
    const DIFFICULTY_LEVEL: Difficulty = Difficulty::Easy;

    fn apply(board: &mut Board) -> bool {
        let mut any_changed = false;
        let max_cells = board.size.max_cells() as usize;
        for i in 0..max_cells {
            for line in [LineKind::Row(i), LineKind::Column(i)] {
                let candidates = line.map_notes_to_coords(board);
                for (candidate, coords) in candidates {
                    let mut it = coords.iter();
                    if let Some(coord) = it.next()
                        && it.len() == 0
                    {
                        board.set_cell(*coord, candidate);
                        any_changed = true;
                    }
                }
            }
        }
        any_changed
    }
}
