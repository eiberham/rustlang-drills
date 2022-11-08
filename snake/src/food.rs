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
  /// Constructs a new food actor.
  pub fn new() -> Self {
    let piece = Tile::rand();
    Self {
      piece
    }
  }

  /// Draws the food on the canvas.
  pub fn draw(&mut self, canvas: &mut Canvas) {

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.piece.draw())
                .color(graphics::Color::WHITE),
        );

  }

  /// Places the food randomly within the playground in an
  /// unoccupied spot
  pub fn serve(&mut self) {
    self.piece = Tile::rand();
  }

}