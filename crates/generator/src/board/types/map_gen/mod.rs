mod cell_finder;
mod region_gen;
use crate::BoardSize;
use region_gen::RegionMap;
use thiserror::Error;

/// Errors that can occur when creating or validating an IrregularMap.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum IrregularMapError {
    /// This typically means the map does not have the correct number of cells
    /// per region, or the overall dimensions are incorrect.
    #[error("The provided map is invalid")]
    InvalidMap,

    /// This typically means that the map has non contiguous regions.
    #[error("The provided map has a non-contiguous region")]
    NonContiguousRegion,
}

/// A mapping of cells to irregular regions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrregularMap {
    pub(super) regions: Vec<Vec<u8>>,
}

impl IrregularMap {
    /// Create a new IrregularMap from a vector of vectors.
    ///
    /// Each inner vector represents a row, and each value in the inner vector
    /// represents the region index for that cell.
    pub fn new(regions: Vec<Vec<u8>>) -> Result<Self, IrregularMapError> {
        let size = regions.len();
        let mut region_map = RegionMap::new(&BoardSize::from(size as u8));
        region_map.regions = regions;
        region_map.is_valid()?;
        Ok(Self {
            regions: region_map.regions,
        })
    }

    pub fn generate(size: &BoardSize) -> Result<Self, IrregularMapError> {
        let mut fake_board = RegionMap::new(size);
        let iterations = (size.max_cells() as u32).pow(size.width() as u32).max(5000);
        fake_board.generate(iterations)?;
        fake_board.is_valid()?;
        Ok(Self {
            regions: fake_board.regions,
        })
    }
}
