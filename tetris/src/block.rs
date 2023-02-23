use ggez::{
  graphics::{ self, Color, Canvas, DrawParam, Mesh, Rect },
  glam::{ Vec2 },
  mint::{ Point2 },
  Context,
  GameError
};

use crate::{ tetromino::* };

#[derive(Clone)]
pub struct Block {
  shape: Shape,
  form: [[u8; 4]; 4],
  orientation: Orientation,
  color: Color,
  rect: Rect
}

impl Block {
  /// Creates a new instance of block
  pub fn new(
    shape: Shape,
    form: [[u8; 4]; 4],
    orientation: Orientation,
    color: Color,
    rect: Rect ) -> Self {
      Self { shape, form, orientation, color, rect }
  }

  /// Tells whether the form index is
  /// filled or not
  pub fn filled(&mut self, x: usize, y: usize) -> bool {
    match self.orientation {
      Orientation::Down => self.form[x][y] == 1,
      _ => false
    }
  }
}

impl Tetromino for Block {
  /// Rotates the tetromino in clockwise
  /// direction
  fn rotate(&mut self) -> () {

  }

  /// Moves the tetromino to the left
  fn move_l(&mut self) -> () {
    self.rect.x -= 32.0;
  }

  /// Moves the tetromino to the right
  fn move_r(&mut self) -> () {
    self.rect.x += 32.0;
  }

  /// Draws the block onto the canvas
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    ctx: &mut Context ) -> Result<(), GameError> {


      for i in 0..4 {
          for j in 0..4 {
              if self.filled(i, j) {
                  canvas.draw(
          &graphics::Quad,
              graphics::DrawParam::new()
                    .dest(self.rect.point())
                    .scale(self.rect.size())
                    .color(self.color),
                  );
              }
          }
      }

      Ok(())
  }

  fn clone_dyn(&self) -> Box<dyn Tetromino> {
    Box::new(self.clone())
  }
}