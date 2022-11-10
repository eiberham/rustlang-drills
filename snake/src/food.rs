//! Food abstraction.
//!
//! Provides an abstraction over a food.
//!

use ggez::graphics::{ self, Canvas };

use crate::tile::*;

/// Represents the food the snake is supposed to eat in order
/// to grow.
///
/// Its piece field will hold up a tile indicating where in
/// the playground the food is
pub struct Food {
  pub piece: Tile
}

impl Food {
  /// Constructs a new food entity.
  ///
  /// Normaly we want the food to appear at a random position,
  /// therefore a call to get_rand is made.
  ///
  /// It will place the food randomly within the playground in
  /// an unoccupied spot
  pub fn new() -> Self {
    let piece = Tile::get_rand();
    Self { piece }
  }

  /// Draws the food over the canvas.
  pub fn draw(&mut self, canvas: &mut Canvas) {
    canvas.draw(&graphics::Quad, graphics::DrawParam::new()
        .dest_rect(self.piece.draw())
        .color(graphics::Color::WHITE),
    );
  }

  /// Places the food randomly within the playground in an
  /// unoccupied spot
  pub fn serve(&mut self) {
    self.piece = Tile::get_rand();
  }

}