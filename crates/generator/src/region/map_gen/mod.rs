mod region_gen;
use crate::BoardSize;
use region_gen::RegionMap;
use thiserror::Error;

/// Errors that can occur when creating or validating an IrregularMap.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum IrregularMapError {
    /// This typically means the map does not have the correct number of cells
    /// for each row and column.
    #[error("The map has invalid dimensions")]
    InvalidMapDimensions,

    /// This typically means that a region index in the map is out of bounds.
    ///
    /// Each region index must be in range `[0, N-1]`, where `N` is the
    /// number of regions (which is also the number of cells in a row or column).
    #[error("The map contains an invalid region index")]
    InvalidRegionIndex,

    /// This typically means that a region in the map does not have the correct
    /// number of cells in it.
    ///
    /// Each region must contain exactly `N` cells, where `N` is the number of
    /// cells in a row or column of the board.
    #[error("The map has a region that does not contain the the expected number of cells")]
    UnbalancedRegion,

    /// This typically means that the map has non contiguous regions.
    ///
    /// Each cell in a region must connect perpendicularly to at least
    /// one other cell in the same region.
    #[error("The map has a non-contiguous region")]
    NonContiguousRegion,
}

/// A mapping of cells to irregular regions.
///
/// Typically this is constructed before being passed to
/// [`RegionKind::Irregular`](crate::RegionKind::Irregular).
///
/// The constructors ensure that the map is valid for the given board size.
/// A valid map has the following properties, assuming the board size is `N x N`:
///
/// - The map contains exactly `N` regions.
/// - Each region contains exactly `N` cells.
/// - Each region is contiguous, meaning all cells in a region are connected perpendicularly.
///
/// # Examples
/// To declare a manually defined irregular map:
/// ```
/// use sudoku_gen::{BoardSize, IrregularMap, RegionKind};
/// let map = IrregularMap::new(vec![
///     vec![0, 0, 0, 1],
///     vec![0, 3, 3, 1],
///     vec![2, 3, 3, 1],
///     vec![2, 2, 2, 1],
/// ]).unwrap();
/// let region_kind = RegionKind::Irregular { map };
/// ```
///
/// To generate a random irregular map:
/// ```
/// use sudoku_gen::{BoardSize, IrregularMap, RegionKind};
/// let map = IrregularMap::generate(&BoardSize::X4).unwrap();
/// let region_kind = RegionKind::Irregular { map };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrregularMap {
    pub(super) regions: Vec<Vec<u8>>,
}

impl IrregularMap {
    /// Create a new IrregularMap from a vector of vectors.
    ///
    /// Each inner vector represents a row, and each value in the inner vectors
    /// represents the region index for that cell.
    /// Therefore, the outer vector's length must match each inner vector's length.
    ///
    /// This constructor will also ensure that the regions are not malformed;
    /// see [`IrregularMap`].
    pub fn new(regions: Vec<Vec<u8>>) -> Result<Self, IrregularMapError> {
        let size = regions.len();
        let mut region_map = RegionMap::new(&BoardSize::from(size as u8));
        region_map.regions = regions;
        region_map.is_valid()?;
        Ok(Self {
            regions: region_map.regions,
        })
    }

    /// Generate a random valid IrregularMap for the given board `size`.
    ///
    /// This starts with a regular region map and performs a series of random
    /// swaps to create an irregular map. For each iteration, a swap is done on
    /// each region of the board. The number of iterations is the
    /// [`BoardSize::max_cells()`] squared, but it is also capped at 100 to
    /// prevent excessive computation time on larger boards (anything bigger than 9x9).
    pub fn generate(size: &BoardSize) -> Result<Self, IrregularMapError> {
        let mut temp_board = RegionMap::new(size);
        let iterations = (size.max_cells() as u32).pow(2).min(100);
        temp_board.generate(iterations)?;
        temp_board.is_valid()?;
        Ok(Self {
            regions: temp_board.regions,
        })
    }
}
