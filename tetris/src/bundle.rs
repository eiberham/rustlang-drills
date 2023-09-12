//! Bundle abstraction.
//!
//! Provides a bundle in charge of holding
//! blocks for the game to draw on every
//! game cycle.
//!

use ggez::{
  graphics::{ self, Color, Canvas, Image },
  Context,
  GameError
};
use crate::{block::*, tetromino::*};

#[derive(Clone, Debug)]
pub struct BundleBlock {
  pub positions: Vec<Position>,
  pub color: Color
}

impl BundleBlock {
  pub fn new(positions: Vec<Position>, color: Color) -> Self {
    Self {
      positions,
      color
    }
  }
}

#[derive(Debug, Clone)]
pub struct Bundle<BundleBlock> {
  pub values: Vec<BundleBlock>
}

impl Bundle<BundleBlock> {
  /// Creates a new bundle instance
  pub fn new() -> Self {
    Self { values:  Vec::new() }
  }

  /// Pushes a block into the bundle
  pub fn push(&mut self, value: BundleBlock) -> () {
    self.values.push(value);
  }

  /// Checks if the bundle is empty
  pub fn is_empty(&self) -> bool {
    self.values.is_empty()
  }

  /// Renders all the blocks within
  /// the bundle
  pub fn render(&self, canvas: &mut Canvas, ctx : &mut Context) -> () {
    let image = Image::from_path(ctx, "/block.png").unwrap();
    for block in self.values.iter() {
      for Position{x, y} in block.positions.iter() {
        canvas.draw(
          &image,
          graphics::DrawParam::new()
            .dest([*x, *y])
            .color(block.color)
          );
      }
    }
  }
}