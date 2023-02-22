//! Factory interface.
//!
//! Provides methods to summon new shaped blocks.
//!

use ggez::{
  graphics::{Color, Rect },
};
use crate::tetromino::{Shape, Tetromino, Orientation};
use crate::block::*;
use crate::shapes::*;

pub trait Factory {
  /// The factory method. It must be overridden with a concrete
  /// implementation.
  /// Trait objects are normal values that store a value of any
  /// type that implements the given trait.
  fn create(shape: Shape) -> Box<dyn Tetromino>;
}

pub struct Builder;

impl Factory for Builder {
  /// It will return a trait object.
  /// Trait objects are normal values that store a value of any
  /// type that implements the given trait
  fn create(shape: Shape) -> Box<dyn Tetromino> {
    Box::new(Block::new(
      shape,
      SHAPE[0][0][0],
      Orientation::Down,
      Color::WHITE,
      Rect::new(0.0, 0.0, 32.0, 32.0)
    ))
  }
}