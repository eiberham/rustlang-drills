use ggez::{
  graphics::{ Color, Canvas, DrawParam, Mesh },
  glam::{ Vec2 },
  Context,
  GameError
};

use crate::{tetromino::*, factory::*};

pub struct I {
  pub position: Position // should be a Point/Vec2 instead
}

impl Factory for I {
  fn create() -> Box<dyn Tetromino> {
    let position = Position::new(0.0, 0.0);
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

  fn render(
    &self,
    canvas: &mut Canvas,
    ctx: &mut Context ) -> Result<(), GameError> {

      let mesh = Mesh::new_line(
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

      canvas.draw(&mesh, DrawParam::new());

      Ok(())
  }
}