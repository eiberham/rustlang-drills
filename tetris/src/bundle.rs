//! Bundle abstraction.
//!
//! Provides a bundle in charge of holding
//! blocks for the game to draw on every
//! game cycle.
//!

use crate::block::*;

#[derive(Debug, Clone)]
pub struct Bundle<Block : Copy> {
  pub values: Vec<Block>
}

impl Bundle<Block> {
  /// Creates a new bundle instance
  pub fn new() -> Self {
    Self { values:  Vec::new() }
  }

  /// Pushes a block into the bundle
  pub fn push(&mut self, value: Block) -> () {
    self.values.push(value);
  }

  /// Checks if the bundle is empty
  pub fn is_empty(&self) -> bool {
    self.values.is_empty()
  }

  /// Renders all the blocks within
  /// the bundle
  pub fn render(&self) -> () {

  }
}