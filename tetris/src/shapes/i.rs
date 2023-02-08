use ggez::{
  graphics::{ self, Color, Canvas, DrawParam, Mesh, Rect },
  glam::{ Vec2 },
  mint::{ Point2 },
  Context,
  GameError
};

use crate::{tetromino::*, factory::*};

pub struct I {
  pub position: Point2<f64>
}

impl Factory for I {
  /// It will return a trait object.
  /// Trait objects are normal values that store a value of any type that implements the given trait
  fn create() -> Box<dyn Tetromino> {
    let position: Point2<f64> = Point2::from([0.0, 0.0]);
    Box::new(Self {
      position
    })
  }
}

impl Tetromino for I {
  /// Rotates the tetromino in clockwise
  /// direction
  fn rotate(&self) -> () {

  }

  /// Moves the tetromino to the left
  fn move_l(&self) -> () {

  }

  /// Moves the tetromino to the right
  fn move_r(&self) -> () {

  }

  fn draw(
    &mut self,
    canvas: &mut Canvas,
    ctx: &mut Context ) -> Result<(), GameError> {

      /* let mesh = Mesh::new_line(
        ctx,
        &[
          Vec2::new(0.0, 0.0),
          Vec2::new(0.0, 32.0),
          Vec2::new(0.0, 64.0),
          Vec2::new(0.0, 96.0),
        ],
        64.0,
        Color::BLUE
      )?;

      canvas.draw(&mesh, DrawParam::new()); */

      let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 64.0, 128.0),
            Color::WHITE,
        )?;

      canvas.draw(&rectangle, DrawParam::new());

      /* let rect = Rect::new(450.0, 450.0, 50.0, 50.0);
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(rect.point())
                .scale(rect.size())
                .color(Color::WHITE),
        ); */

      Ok(())
  }
}