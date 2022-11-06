use ggez::{ mint };

use crate::place::*;

pub struct Food {
  place: Place
}

impl Food {
  pub fn new() -> Self {
    let place = Place::get();
    Self { place }
  }

  pub fn draw(&mut self, canvas: &mut Canvas) {
    //randomly places the food within the playground

    let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2{x: 200.0, y: 300.0},
            100.0,
            0.1,
            graphics::Color::WHITE,
        )?;

    canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(*square)
                .color(graphics::Color::GREEN),
        );

  }

  pub fn fade(&mut self) {
    // makes the food disappear from the playground
  }

}