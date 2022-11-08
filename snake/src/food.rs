use ggez::graphics::{ self, Canvas };

use crate::tile::*;

pub struct Food {
  pub piece: Tile
}

impl Food {
  pub fn new() -> Self {
    let piece = Tile::rand();
    Self {
      piece
    }
  }

  pub fn draw(&mut self, canvas: &mut Canvas) {

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.piece.draw())
                .color(graphics::Color::WHITE),
        );

  }

  pub fn serve(&mut self) {
    self.piece = Tile::rand();
  }

}