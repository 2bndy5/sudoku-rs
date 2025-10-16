use std::{collections::HashMap, fmt::Display};

use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{
    BoardSize,
    cell::{Cardinal, Coord},
};

use super::IrregularMapError;

/// A struct to generate and validate an irregular region map.
#[derive(Debug, Clone)]
pub struct RegionMap {
    /// The mapping of cells to regions.
    pub(super) regions: Vec<Vec<u8>>,

    /// The constraints of the board.
    size: BoardSize,

    /// A random number generator (only for generating the map).
    rng: ThreadRng,

    /// The current region being processed (only used when generating the map).
    current_region: u8,
}

type BorderingRegion = (u8, u8); // (region, bordering region)

impl RegionMap {
    pub fn new(size: &BoardSize) -> Self {
        let max_cells = size.max_cells();
        let width = size.width();
        let height = size.height();
        let mut regions = vec![vec![u8::MAX; max_cells as usize]; max_cells as usize];
        for i in 0..max_cells {
            let row_offset = i / height * height;
            let col_offset = i % height * width;
            for j in 0..max_cells {
                let row = j / width + row_offset;
                let col = j % width + col_offset;
                regions[row as usize][col as usize] = i;
            }
        }
        Self {
            regions,
            size: size.clone(),
            rng: rand::rng(),
            current_region: 0,
        }
    }

    pub fn is_valid(&self) -> Result<(), IrregularMapError> {
        let max_cells = self.size.max_cells() as usize;
        let mut counts = vec![0; max_cells];
        for row in &self.regions {
            if row.len() != max_cells {
                return Err(IrregularMapError::InvalidMapDimensions);
            }
            for col in row {
                if *col >= max_cells as u8 {
                    return Err(IrregularMapError::InvalidRegionIndex);
                }
                counts[*col as usize] += 1;
            }
        }
        // verify counts (of cells per region) match max number of cells
        for count in counts {
            if count != max_cells {
                return Err(IrregularMapError::UnbalancedRegion);
            }
        }
        for region in 0..max_cells {
            if !self.check_contiguity(region as u8) {
                return Err(IrregularMapError::NonContiguousRegion);
            }
        }
        Ok(())
    }

    fn swap_cells(
        &mut self,
        regions: &BorderingRegion,
        adjacent: &(Vec<Coord>, Vec<Coord>),
    ) -> bool {
        for cell1 in &adjacent.0 {
            for cells2 in &adjacent.1 {
                if self.try_swap_cells(regions, *cell1, *cells2) {
                    return true;
                }
            }
        }
        false
    }

    fn try_swap_cells(&mut self, regions: &BorderingRegion, cell1: Coord, cell2: Coord) -> bool {
        let max_cells = self.size.max_cells() as usize;
        let (region1, region2) = regions;
        if region1 == region2 {
            return false;
        }
        if cell1.row < max_cells
            && cell2.row < max_cells
            && cell1.col < max_cells
            && cell2.col < max_cells
        {
            // Swapped cells' region
            self.regions[cell1.row][cell1.col] = *region2;
            self.regions[cell2.row][cell2.col] = *region1;
            // Checking continuity of both regions
            if !self.check_contiguity(*region1) || !self.check_contiguity(*region2) {
                // Failed; reverting swap
                self.regions[cell1.row][cell1.col] = *region1;
                self.regions[cell2.row][cell2.col] = *region2;
                return false;
            }
            return true;
        }
        false
    }

    fn get_region_borders(&self, region: u8) -> HashMap<BorderingRegion, (Vec<Coord>, Vec<Coord>)> {
        let mut neighbors: HashMap<(u8, u8), (Vec<Coord>, Vec<Coord>)> = HashMap::new();
        let max_cells = self.size.max_cells() as usize;
        for row in 0..max_cells {
            for col in 0..max_cells {
                if self.regions[row][col] != region {
                    continue;
                }
                // found a cell in our desired region.
                // look for adjacent cells in bordering regions
                let cell = Coord { row, col };
                for coord in Cardinal::get_neighbors(max_cells - 1, cell) {
                    if self.regions[coord.row][coord.col] != region {
                        let key = (region, self.regions[coord.row][coord.col]);
                        let proposed = (cell, coord);
                        if let Some(lists) = neighbors.get_mut(&key) {
                            if !lists.0.contains(&proposed.0) {
                                lists.0.push(proposed.0);
                            }
                            if !lists.1.contains(&proposed.1) {
                                lists.1.push(proposed.1);
                            }
                        } else {
                            neighbors.insert(key, (vec![proposed.0], vec![proposed.1]));
                        }
                    }
                }
            }
        }
        neighbors
    }

    fn get_region_cells(&self, region: u8) -> Vec<Coord> {
        let mut cells = Vec::new();
        let max_cells = self.size.max_cells() as usize;
        for row in 0..max_cells {
            for col in 0..max_cells {
                if self.regions[row][col] == region {
                    cells.push(Coord { row, col });
                }
            }
        }
        cells
    }

    fn check_contiguity(&self, region: u8) -> bool {
        let cells = self.get_region_cells(region);
        // cells.len() should always be > 0 because of Self::new()

        let max_cells = self.size.max_cells() as usize;
        // parallel to `cells` for tracking visited cells
        let mut visited = vec![false; cells.len()];
        // A queue of cells to trace for continuity.
        // This is expanded as cells and their connected neighbors are found
        let mut to_visit = vec![cells[0]];

        while let Some(cell) = to_visit.pop() {
            // safe to `position().unwrap()` because we know `cell` is in `cells`
            let cell_index = cells.iter().position(|&c| c == cell).unwrap();
            visited[cell_index] = true;
            for neighbor in Cardinal::get_neighbors(max_cells - 1, cell) {
                if cells.contains(&neighbor) {
                    // safe to `position().unwrap()` because we know `neighbor` is in `cells`
                    let neighbor_index = cells.iter().position(|&c| c == neighbor).unwrap();
                    if !visited[neighbor_index] {
                        to_visit.push(neighbor);
                    }
                }
            }
        }

        let count = visited.iter().filter(|v| **v).count();
        count == cells.len()
    }

    pub fn generate(&mut self, iterations: u32) -> Result<(), IrregularMapError> {
        for _ in 0..iterations {
            let neighbors = self.get_region_borders(self.current_region);
            // try neighbors in random order
            let mut keys = neighbors.keys().collect::<Vec<&BorderingRegion>>();
            keys.shuffle(&mut self.rng);
            // neighbors.shuffle(&mut self.rng);
            for adjacent in keys {
                if self.swap_cells(adjacent, &neighbors[adjacent]) {
                    break;
                }
            }
            self.current_region = if self.current_region == self.size.max_cells() - 1 {
                0
            } else {
                self.current_region + 1
            };
        }
        Ok(())
    }
}

impl Display for RegionMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.regions {
            for (i, col) in row.iter().enumerate() {
                let suffix = if i == row.len() - 1 { "\n" } else { " " };
                write!(f, "{col:X}{suffix}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(size: BoardSize) {
        let max_cells = size.max_cells();
        let iterations = (max_cells as u32).pow(3 as u32).min(1000);
        let mut map = RegionMap::new(&size);
        map.generate(iterations).unwrap();
        print!("{map}");
        map.is_valid().unwrap();
        println!("Valid {max_cells}x{max_cells} map generated using {iterations} iterations");
    }

    #[test]
    fn generate_4x4() {
        generate(BoardSize::X4);
    }

    #[test]
    fn generate_6x6() {
        generate(BoardSize::X6);
    }

    #[test]
    fn generate_9x9() {
        generate(BoardSize::X9);
    }

    #[test]
    fn generate_12x12() {
        generate(BoardSize::X12);
    }

    #[test]
    fn generate_16x16() {
        generate(BoardSize::X16);
    }

    #[test]
    fn map_bad_rows() {
        let mut map = RegionMap::new(&BoardSize::X4);
        // manually create an invalid map with wrong number of rows
        map.regions = vec![vec![0, 0, 1], vec![0, 0, 1], vec![2, 2, 3]];
        assert_eq!(map.is_valid(), Err(IrregularMapError::InvalidMapDimensions));
    }

    #[test]
    fn map_bad_cols() {
        let mut map = RegionMap::new(&BoardSize::X4);
        // manually create an invalid map with wrong number of columns
        map.regions = vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3],
            vec![2, 2, 3],
        ];
        assert_eq!(map.is_valid(), Err(IrregularMapError::InvalidMapDimensions));
    }

    #[test]
    fn map_bad_region_index() {
        let mut map = RegionMap::new(&BoardSize::X4);
        // manually create an invalid map with out-of-bounds region index
        map.regions = vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3],
            vec![2, 2, 3, 4],
        ];
        assert_eq!(map.is_valid(), Err(IrregularMapError::InvalidRegionIndex));
    }

    #[test]
    fn map_bad_region_count() {
        let mut map = RegionMap::new(&BoardSize::X4);
        // manually create an invalid map with wrong number of cells per region
        map.regions = vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3],
            vec![2, 2, 2, 2],
        ];
        assert_eq!(map.is_valid(), Err(IrregularMapError::UnbalancedRegion));
    }

    #[test]
    fn map_non_contiguous_regions() {
        let mut map = RegionMap::new(&BoardSize::X4);
        // manually create a non-contiguous region
        let non_continuous_map = vec![
            vec![0, 0, 0, 1],
            vec![0, 3, 1, 1],
            vec![3, 2, 1, 2],
            vec![3, 2, 2, 3],
        ];
        map.regions = non_continuous_map.clone();
        assert_eq!(map.is_valid(), Err(IrregularMapError::NonContiguousRegion));
        assert!(map.check_contiguity(0));
        assert!(map.check_contiguity(1));
        assert!(!map.check_contiguity(2));
        assert!(!map.check_contiguity(3));

        // try to swap a cell from region 0 with a cell from region 1
        let cell1 = Coord { row: 3, col: 2 }; // region 0
        let cell2 = Coord { row: 3, col: 3 }; // region 1
        assert!(!map.try_swap_cells(&(2, 3), cell1, cell2));
        // swapping with invalid region indexes
        assert!(!map.try_swap_cells(&(2, 2), cell1, cell2));
        // swapping with out-of-bounds coordinates
        assert!(!map.try_swap_cells(&(2, 3), cell1, Coord { row: 4, col: 3 }));
        assert!(!map.try_swap_cells(&(2, 3), cell1, Coord { row: 3, col: 4 }));
        // regions should be unchanged
        assert_eq!(map.regions, non_continuous_map);
    }
}
