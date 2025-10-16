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
    pub fn related_regions(&self, coord: Coord) -> BTreeMap<u8, Vec<Coord>> {
        let mut regions: BTreeMap<u8, Vec<Coord>> = BTreeMap::new();
        let max_cells = self.size.max_cells() as usize - 1;
        if let Some(region) = self.kind.index(&self.size, coord) {
            regions.insert(region, self.kind.members(&self.size, region));
            let walkers = [
                // walker for row
                CoordWalker {
                    origin: Coord {
                        row: coord.row,
                        col: 0,
                    },
                    direction: Cardinal::East,
                    max_index: max_cells,
                },
                // walker for column
                CoordWalker {
                    origin: Coord {
                        row: 0,
                        col: coord.col,
                    },
                    direction: Cardinal::South,
                    max_index: max_cells,
                },
            ];
            for walker in walkers {
                for c in walker {
                    let reg = self.kind.index(&self.size, c);
                    if reg.is_none_or(|r| r == region) {
                        continue;
                    }
                    // safe to reg.unwrap() because of the is_none_or() check above
                    regions
                        .entry(reg.unwrap())
                        .or_insert_with(|| self.kind.members(&self.size, reg.unwrap()));
                }
            }
        }
        regions
    }
}

#[cfg(test)]
mod test {
    use crate::{Board, BoardSize, RegionKind, cell::Coord};

    #[test]
    fn test_related_regions() {
        let board = Board::new(&BoardSize::X4, RegionKind::Regular);
        let origin = Coord { row: 1, col: 1 };
        let regions = board.related_regions(origin);
        assert_eq!(regions.len(), 3);
        // expected regions for a 4x4 regular board are conveniently the first 3 regions
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
