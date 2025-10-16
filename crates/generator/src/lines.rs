//! Utilities for analyzing a single row or column
#![allow(dead_code)]

use crate::{Board, Cardinal, Cell, Coord, CoordWalker};
use std::collections::{BTreeMap, HashMap, HashSet};

/// A kind of line in the Sudoku board: either a row or a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    /// A row at the given index.
    Row(usize),
    /// A column at the given index.
    Column(usize),
}

impl LineKind {
    /// Create a map of candidates to the coordinates that contain them.
    ///
    /// Coordinates are specific to the instantiated line (`self`).
    pub fn map_notes_to_coords(&self, board: &Board) -> BTreeMap<u8, HashSet<Coord>> {
        let (origin, direction) = match self {
            LineKind::Row(line_index) => (
                Coord {
                    row: *line_index,
                    col: 0,
                },
                Cardinal::East,
            ),
            LineKind::Column(line_index) => (
                Coord {
                    row: 0,
                    col: *line_index,
                },
                Cardinal::South,
            ),
        };
        let walker = CoordWalker {
            origin,
            direction,
            max_index: board.size.max_cells() as usize - 1,
        };
        let mut map = BTreeMap::new();
        if let Cell::Notes(notes) = &board.grid[origin.row][origin.col] {
            for note in notes {
                let entry = map.entry(*note).or_insert(HashSet::new());
                entry.insert(origin);
            }
        }
        for coord in walker {
            if let Cell::Notes(notes) = &board.grid[coord.row][coord.col] {
                for note in notes {
                    let entry = map.entry(*note).or_insert(HashSet::new());
                    entry.insert(coord);
                }
            }
        }
        map
    }

    /// Find cells that share the same candidates.
    ///
    /// Returns a map of shared candidates to the set of coordinates that share them.
    /// Coordinates are specific to the instantiated line (`self`).
    pub fn find_cells_with_shared_candidates(
        &self,
        board: &Board,
    ) -> HashMap<Vec<u8>, HashSet<Coord>> {
        let notes_map = self.map_notes_to_coords(board);
        let mut shared_notes: HashMap<Vec<u8>, HashSet<Coord>> = HashMap::new();
        for (note, coords) in &notes_map {
            let visiting_coords_len = coords.len();
            if visiting_coords_len > 1 {
                let mut shared_candidates = vec![*note];
                let visiting_coords: HashSet<Coord> = coords.iter().copied().collect();
                for (other_note, other_coords) in &notes_map {
                    if other_note == note || other_coords.len() != visiting_coords_len {
                        continue;
                    }
                    // visiting_coords vs other_coords may be out of order,
                    // so use intersection and compare lengths
                    let intersection = visiting_coords
                        .intersection(other_coords)
                        .collect::<HashSet<&Coord>>();
                    if visiting_coords_len == intersection.len() {
                        // found some shared candidates
                        shared_candidates.push(*other_note);
                    }
                }
                // only keep if number of candidates matches the number of shared cells
                if shared_candidates.len() == visiting_coords_len {
                    // avoid duplicates by keeping the shared candidates (key) sorted
                    shared_candidates.sort();
                    shared_notes
                        .entry(shared_candidates)
                        .or_insert(visiting_coords);
                }
            }
        }
        // if !shared_notes.is_empty() {
        //     println!("notes_map:");
        //     for (note, coords) in notes_map {
        //         println!(
        //             "\t{note}: [{}]",
        //             coords
        //                 .iter()
        //                 .map(|c| format!("{c}"))
        //                 .collect::<Vec<String>>()
        //                 .join(", ")
        //         );
        //     }
        //     for (candidates, coords) in &shared_notes {
        //         println!(
        //             "shared_notes:\t{candidates:?}: [{}]",
        //             coords
        //                 .iter()
        //                 .map(|c| format!("{c}"))
        //                 .collect::<Vec<String>>()
        //                 .join(", ")
        //         );
        //     }
        // }
        shared_notes
    }
}
