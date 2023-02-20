use ggez::{
  graphics::{ self, Color, Canvas, DrawParam, Mesh, Rect },
  glam::{ Vec2 },
  mint::{ Point2 },
  Context,
  GameError
};


use crate::{ tetromino::* };

pub struct Block {
  shape: Shape,
  form: [[i32;10]; 20],
  orientation: Orientation,
  color: Color
}

impl Block {
  /// Creates a new instance of block
  pub fn new(
    shape: Shape,
    form: [[i32;10]; 20],
    orientation: Orientation,
    color: Color ) -> Self {
      Self { shape, form, orientation, color }
  }
}

impl Tetromino for Block {
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

  /// Draws the block onto the canvas
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

  fn clone(&self) -> Box<dyn Tetromino> {
    Box::new(self.clone()) // Forward to the derive(Clone) impl
  }
}