#![allow(clippy::doc_lazy_continuation)]
#![doc = include_str!("../README.md")]
mod cell;
pub use cell::{Cell, Coord};

mod board;
pub use board::{Board, BoardKind, BoardSize, IrregularMap, IrregularMapError, ParseError};

mod generate;
mod solve;
mod validate;
pub use validate::ValidationResults;
