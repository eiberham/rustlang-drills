//! Block element.
//!
//! Provides an entity that will represent the
//! blocks or tetromioes around the game.
//!

use ggez::{
  graphics::{ self, Color, Canvas, Image },
  Context,
  GameError
};

use crate::{ tetromino::*, bundle::* };

#[derive(Clone, Copy, Debug)]
pub struct Block {
  pub shape: Shape,
  pub form: [[[u8; 4]; 4]; 4],
  pub orientation: Orientation,
  pub color: Color,
  pub position: Position
}

impl Block {
  /// Creates a new instance of block / tetromino
  pub fn new(
    shape: Shape,
    form: [[[u8; 4]; 4]; 4],
    orientation: Orientation,
    color: Color,
    position: Position ) -> Self {
      Self { shape, form, orientation, color, position }
  }

  /// Tells whether the form index is filled
  pub fn filled(&self, x: usize, y: usize) -> bool {
    let w = self.shape.current(self.orientation);
    self.form[w][x][y] == 1
  }

  /// Retrieves the corresponding position tiles
  pub fn tiles(&self) -> Vec<Position> {
    let mut tiles: Vec<Position> = Vec::new();
    for i in 0..4 {
      for j in 0..4 {
        if self.filled(j, i) {
          let x: f32 = self.position.x + ((i as f32 + 1.0) * 32.0);
          let y: f32 = self.position.y + ((j as f32 + 1.0) * 32.0);
          tiles.push(Position::new(x, y))
        }
      }
    }
    tiles
  }

  /// Checks if the block has reached the bottom
  /// of the board
  pub fn landed(&self) -> bool {
    self.tiles().into_iter()
          .any(|position: Position| position.y == 928.0)
  }

  /// Checks if the block collided with any landed
  /// block
  pub fn collides(&self, bundle: Bundle<Block>) -> bool {
    self.tiles()
        .into_iter()
        .any(|p|{
          bundle.values.iter()
                       .any(|&b| {
                          b.tiles()
                           .into_iter()
                           .any(|t: Position| t == Position::new(p.x, p.y + 32.0))
                        })
        }
    )
  }
}

impl Tetromino for Block {
  /// Rotates the tetromino in clockwise
  /// direction
  fn rotate(&mut self) -> () {
    let rotation: [Orientation; 4] = [
      Orientation::Up,
      Orientation::Right,
      Orientation::Down,
      Orientation::Left
    ];

    let i: usize = rotation
      .iter()
      .position(|x: &Orientation| *x == self.orientation)
      .unwrap();

    let index: usize =  if i == rotation.len() -1 { 0 } else { i + 1 };
    self.orientation = rotation[index]
  }

  /// Moves the tetromino to the left
  fn move_l(&mut self) -> () {
    let tiles: Vec<Position> = self.tiles();
    if !tiles.into_iter()
             .any(|position| position.x == 0. ) {
      self.position.x -= 32.0;
    }
  }

  /// Moves the tetromino to the right
  fn move_r(&mut self) -> () {
    let tiles: Vec<Position> = self.tiles();
    if !tiles.into_iter()
             .any(|position| position.x == 384. ) {
      self.position.x += 32.0;
    }
  }

  /// Moves the tetromino to the bottom
  fn move_d(&mut self) -> () {
    self.position.y += 32.0;
  }

  /// Draws the block onto the canvas
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    ctx: &mut Context ) -> Result<(), GameError> {
      let image = Image::from_path(ctx, "/block.png").unwrap();
      let tiles: Vec<Position> = self.tiles();
      for Position {x, y} in tiles {
        canvas.draw(
          &image,
          graphics::DrawParam::new()
            .dest([x, y])
            .color(self.color)
          );
      }
      Ok(())
  }
}

/// Allows later equal comparison between
/// block elements
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.shape == other.shape
    }
}