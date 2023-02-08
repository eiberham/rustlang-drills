use crate::tetromino::{self, Shape, Tetromino};

pub trait Factory {
  /// The factory method. It must be overridden with a concrete implementation.
  /// Trait objects are normal values that store a value of any type that implements
  /// the given trait.
  fn create() -> Box<dyn Tetromino>;
}