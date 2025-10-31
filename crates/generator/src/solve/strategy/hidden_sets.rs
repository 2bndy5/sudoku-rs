use std::collections::HashSet;

use super::Strategy;
use crate::{Board, Cell, Difficulty, LineKind};

pub struct HiddenSets;

impl Strategy for HiddenSets {
    const DIFFICULTY_LEVEL: Difficulty = Difficulty::Medium;

    /// Eliminate candidates using locked candidates technique (type 2).
    ///
    /// This technique is like an inversion of the locked candidates type 1 technique.
    /// Also known as "hidden pairs/triples/quads".
    /// We look for common candidates shared among cells in the same row or column,
    /// and eliminates any candidate(s) that are not shared among the cells that
    /// have shared candidates.
    ///
    /// This is done separately from `use_region_candidates()`
    /// because it requires a board-wide scope. Whereas, locked candidates type 1
    /// can be done within the scope of a single region.
    fn apply(board: &mut Board) -> bool {
        let max_cells = board.size.max_cells() as usize;
        let mut any_changed = false;
        for i in 0..max_cells {
            for line_kind in [LineKind::Row(i), LineKind::Column(i)] {
                let shared_candidates = line_kind.find_cells_with_shared_candidates(board);
                // eliminate non-shared candidates from cells that share candidates
                for (candidates, coords) in shared_candidates {
                    let mut region_index = None;
                    let mut same_region: Option<bool> = None;
                    for coord in &coords {
                        // check if the cell is in the same region as previous cells
                        match same_region {
                            Some(is_same) if is_same => {
                                // done for consecutive cells while region remains the same
                                let cell_region = board.kind.index(&board.size, *coord);
                                if region_index != cell_region {
                                    // the region varies for one or more cells
                                    same_region = Some(false); // stop further comparisons
                                }
                            }
                            None => {
                                // only done for first cell
                                region_index = board.kind.index(&board.size, *coord);
                                same_region = Some(true);
                            }
                            _ => {} // already determined to be different regions
                        }
                        if let Cell::Notes(notes) = &board.grid[coord.row][coord.col]
                            && notes.len() != candidates.len()
                        {
                            // println!("notes for {coord}: {notes:?} became {candidates:?}");
                            any_changed = true;
                            board.grid[coord.row][coord.col] =
                                Cell::Notes(HashSet::from_iter(candidates.clone()));
                        }
                    }
                    if let Some(region_index) = region_index
                        && same_region.is_some_and(|v| v)
                    {
                        // all cells that share candidates are in the same region
                        // we can eliminate the candidates from other cells in the region
                        // that do not share the candidates
                        // println!("locked candidates {candidates:?} in region {region_index}");
                        let region = board.kind.members(&board.size, region_index);
                        for coord in region {
                            if !coords.contains(&coord)
                                && let Cell::Notes(notes) = &mut board.grid[coord.row][coord.col]
                                && notes.iter().any(|n| candidates.contains(n))
                            {
                                any_changed = true;
                                notes.retain(|n| !candidates.contains(n));
                                // println!("notes for {coord}: {notes:?} became {candidates:?}");
                            }
                        }
                    }
                }
            }
        }
        any_changed
    }
}
