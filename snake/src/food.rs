//! Food abstraction.
//!
//! Provides an abstraction over a food.
//!

use ggez::{
  Context,
  graphics::{ Image, DrawParam, Canvas, Rect }
};
use crate::tile::*;

/// Represents the food the snake is supposed to eat in order
/// to grow.
///
/// Its piece field will hold up a tile indicating where in
/// the playground the food is
#[derive(smart_default::SmartDefault)]
pub struct Food {
  #[default(Tile::get_rand())]
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
    Self::default()
  }

  /// Draws the food over the canvas.
  /// It takes into account the position of the food within the
  /// sprite image.
  pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) {
    let image = Image::from_path(ctx, "/sprite.png").unwrap();
    canvas.draw(&image, DrawParam::new()
        .src(Rect::new(
            0.0,
            0.0,
            1.0 / 6.0,
            1.0)
        )
        .dest([self.piece.x, self.piece.y])
    );
  }

  /// Places the food randomly within the playground in an spot
  /// luckily unoccupied by the snake.
  pub fn serve(&mut self) {
    self.piece = Tile::get_rand();
  }

}