use crate::tetromino::{self, Shape, Tetromino};

pub trait Factory {
  /// The factory method. It must be overridden with a concrete implementation.
  fn create() -> Box<dyn Tetromino>;
}