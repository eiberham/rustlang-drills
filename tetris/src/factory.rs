//! Factory interface.
//!
//! Provides methods to summon new shaped blocks.
//!

use crate::tetromino::{Shape, Position, Orientation, Direction};
use crate::block::*;

pub trait Factory {
  /// The factory method. It must be overridden with a concrete
  /// implementation.
  fn create(shape: Shape) -> Block;
}

pub struct Piece;

impl Factory for Piece {
  fn create(shape: Shape) -> Block {
    Block::new(
      shape,
      shape.matrix(),
      Direction::D,
      Orientation::Down,
      shape.color(),
      Position::new(128., 0.)
    )
  }
}