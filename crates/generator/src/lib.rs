#![allow(clippy::doc_lazy_continuation)]
#![doc = include_str!("../README.md")]
mod cell;
pub use cell::{Cardinal, Cell, Coord, CoordWalker};

mod board;
pub use board::{Board, ParseError};
mod size;
pub use size::BoardSize;
mod region;
pub use region::{IrregularMap, IrregularMapError, RegionKind};
mod lines;
pub use lines::LineKind;

mod generate;
mod solve;
mod validate;
pub use validate::ValidationResults;
