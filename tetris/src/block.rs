use std::ops::Deref;
use ggez::{
  graphics::{ self, Color, Canvas, DrawParam, Mesh, Rect },
  glam::{ Vec2 },
  mint::{ Point2 },
  Context,
  GameError
};

use crate::{ tetromino::* };

#[derive(Clone, Copy, Debug)]
pub struct Block {
  pub shape: Shape,
  pub form: [[u8; 4]; 4],
  pub orientation: Orientation,
  pub color: Color,
  pub position: Position
}

impl Block {
  /// Creates a new instance of block
  pub fn new(
    shape: Shape,
    form: [[u8; 4]; 4],
    orientation: Orientation,
    color: Color,
    position: Position ) -> Self {
      Self { shape, form, orientation, color, position }
  }

  /// Tells whether the form index is
  /// filled or not
  pub fn filled(&self, x: usize, y: usize) -> bool {
    // println!("{}", self.form[x][y]);
    let r: bool = self.form[x][y] == 1;
    r
  }
}

impl Tetromino for Block {
  /// Rotates the tetromino in clockwise
  /// direction
  fn rotate(&mut self) -> () {

  }

  /// Moves the tetromino to the left
  fn move_l(&mut self) -> () {
    self.position.x -= 32.0;
    // println!("{:?}", self.position);
  }

  /// Moves the tetromino to the right
  fn move_r(&mut self) -> () {
    self.position.x += 32.0;
    // println!("{:?}", self.position);
  }

  /// Draws the block onto the canvas
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    ctx: &mut Context ) -> Result<(), GameError> {
      for i in 0..4 {
          for j in 0..4 {
              if self.filled(i, j) {
                // println!("i: {}, j: {}", i, j);
                  let x: f32 = self.position.x + ((i as f32 + 1.0) * 32.0);
                  let y: f32 = self.position.y + ((j as f32 + 1.0) * 32.0);
                  canvas.draw(
          &graphics::Quad,
              graphics::DrawParam::new()
                    .dest([x, y])
                    .scale([32., 32.])
                    .color(self.color)
                  );
              }
          }
      }
      Ok(())
  }

  /* fn clone_dyn(&self) -> Box<dyn Tetromino> {
    Box::new(self.clone())
  } */
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.shape == other.shape
    }
}

/* impl Deref for Block {
    type Target = Self;

    fn deref(&self) -> &Self {
        &self
    }
} */