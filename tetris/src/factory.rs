use crate::{ tetromino };

pub trait Factory {
  /// The factory method. It must be overridden with a concrete implementation.
  fn create(shape: Shape) -> Box<dyn Tetromino>;
}