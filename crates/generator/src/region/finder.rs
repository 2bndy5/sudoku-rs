use std::collections::BTreeMap;

use crate::{
    Board,
    cell::{Cardinal, Coord, CoordWalker},
};

impl Board {
    /// Find all regions related to the given coordinate.
    ///
    /// Includes the region that contains the given coordinate, as well as any regions
    /// that share the same row or column.
    ///
    /// Returns a map of region index to the list of coordinates that belong to that region.
    /// If the given coordinate is out of bounds, then an empty map is returned.
    pub fn related_regions(&self, coord: Coord) -> BTreeMap<u8, Vec<Coord>> {
        let mut regions: BTreeMap<u8, Vec<Coord>> = BTreeMap::new();
        let max_cells = self.size.max_cells() as usize - 1;
        if coord.row > max_cells || coord.col > max_cells {
            return regions;
        }
        let walkers = [
            // walker for row
            CoordWalker::new(
                Coord {
                    row: coord.row,
                    col: 0,
                },
                Cardinal::East,
                max_cells,
            ),
            // walker for column
            CoordWalker::new(
                Coord {
                    row: 0,
                    col: coord.col,
                },
                Cardinal::South,
                max_cells,
            ),
        ];
        for walker in walkers {
            for c in walker {
                let reg = self.kind.index(&self.size, c);
                if reg.is_none_or(|r| regions.contains_key(&r)) {
                    continue;
                }
                // safe to reg.unwrap() because of the is_none_or() check above
                regions
                    .entry(reg.unwrap())
                    .or_insert_with(|| self.kind.members(&self.size, reg.unwrap()));
            }
        }
        regions
    }
}

#[cfg(test)]
mod test {
    use crate::{Board, BoardSize, Coord, RegionKind};

    #[test]
    fn test_related_regions() {
        let board = Board::new(&BoardSize::X4, RegionKind::Regular);
        let max_cells = board.size.max_cells() as usize;

        // test row out of bounds
        let invalid_coord = Coord {
            row: max_cells,
            ..Default::default()
        };
        let regions = board.related_regions(invalid_coord);
        assert!(regions.is_empty());

        // test column out of bounds
        let invalid_coord = Coord {
            col: max_cells,
            ..Default::default()
        };
        let regions = board.related_regions(invalid_coord);
        assert!(regions.is_empty());

        // test valid coordinate
        let origin = Coord::default(); // (0,0)
        let regions = board.related_regions(origin);
        assert_eq!(regions.len(), 3);
        // expected regions (related to origin) for a 4x4 regular board are regions 0-2
        let expected = vec![
            board.kind.members(&board.size, 0),
            board.kind.members(&board.size, 1),
            board.kind.members(&board.size, 2),
        ];
        for (region, members) in regions {
            assert_eq!(expected[region as usize], members);
        }
    }
}
