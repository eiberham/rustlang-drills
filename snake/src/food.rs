use ggez::{ mint };
use ggez::graphics::{ self, Rect, Canvas };

use crate::place::*;

pub struct Food {
  pub place: Place
}

impl Food {
  pub fn new() -> Self {
    let place = Place::get();
    Self { place }
  }

  pub fn draw(&mut self, canvas: &mut Canvas) {

    let square = Rect::new(self.place.x, self.place.y, 0x20 as f32, 0x20 as f32);

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(square)
                .color(graphics::Color::WHITE),
        );

  }

  pub fn serve(&mut self) {
    self.place = Place::get();
  }

}