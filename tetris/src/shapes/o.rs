use ggez::{
  graphics::{ self, Color, Canvas, DrawParam, Mesh, Rect },
  glam::{ Vec2 },
  mint::{ Point2 },
  Context,
  GameError
};

use crate::{tetromino::*, factory::*};

pub struct O {
  pub position: Point2<f64>,
  shape: Shape,
  form: [[i32;4]; 4]
}

impl Factory for O {
  /// It will return a trait object.
  /// Trait objects are normal values that store a value of any type that implements the given trait
  fn create() -> Box<dyn Tetromino> {
    let position: Point2<f64> = Point2::from([0.0, 0.0]);
    Box::new(Self {
      position,
      shape: Shape::O,
      form: [[0; 4]; 4]
    })
  }
}

impl Tetromino for O {
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

      let rect = Rect::new(64.0, 0.0, 64.0, 64.0);
      canvas.draw(
          &graphics::Quad,
          graphics::DrawParam::new()
              .dest(rect.point())
              .scale(rect.size())
              .color(Color::WHITE),
      );

      Ok(())
  }
}