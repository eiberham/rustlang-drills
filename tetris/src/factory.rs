//! Factory interface.
//!
//! Provides methods to summon new shaped blocks.
//!

use ggez::{
  graphics::{Color, Rect },
};
use crate::tetromino::{Shape, Position, Orientation};
use crate::block::*;
use crate::squares::*;

pub trait Factory {
  /// The factory method. It must be overridden with a concrete
  /// implementation.
  /// Trait objects are normal values that store a value of any
  /// type that implements the given trait.
  //fn create(shape: Shape) -> Box<dyn Tetromino>;
  fn create(shape: Shape) -> Block;
}

pub struct Builder;

impl Factory for Builder {
  /// It will return a trait object.
  /// Trait objects are normal values that store a value of any
  /// type that implements the given trait
  fn create(shape: Shape) -> Block { // Box<dyn Tetromino>
    Block::new(
      shape,
      squares[0][0],
      Orientation::Down,
      Color::WHITE,
      Position::new(0., 0.)
    )
  }
}