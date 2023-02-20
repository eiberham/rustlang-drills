use ggez::{
  graphics::{ self, Color },
};
use crate::tetromino::{self, Shape, Tetromino, Orientation};
use crate::block::*;

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
    // let position: Point2<f64> = Point2::from([0.0, 0.0]);
    Box::new(Block::new(
      shape,
      [[0; 10]; 20],
      Orientation::Down,
      Color::WHITE
    ))
  }
}