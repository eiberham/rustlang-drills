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

use crate::{
  utils::{ Position, Direction, Orientation, Shape, Tetromino },
  board::{ self, Board }
};

#[derive(Clone, Copy, Debug)]
pub struct Block {
  pub shape: Shape,
  pub form: [[[u8; 4]; 4]; 4],
  pub direction: Direction,
  pub orientation: Orientation,
  pub color: Color,
  pub position: Position
}

impl Block {
  /// Creates a new instance of block / tetromino
  pub fn new(
    shape: Shape,
    form: [[[u8; 4]; 4]; 4],
    direction: Direction,
    orientation: Orientation,
    color: Color,
    position: Position ) -> Self {
      Self { shape, form, direction, orientation, color, position }
  }

  /// Tells whether the form index is filled
  pub fn filled(&self, x: usize, y: usize) -> bool {
    let w = self.shape.current(self.orientation);
    self.form[w][x][y] == 1
  }

  /// Retrieves the corresponding position tiles
  pub fn get_positions(&self) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let Position { x, y } = self.position;
    for i in 0..4 {
      for j in 0..4 {
        if self.filled(j, i) {
          let x: f32 = x + ((i as f32 + 1.) * 32.);
          let y: f32 = y + ((j as f32 + 1.) * 32.);
          positions.push(Position::new(x, y))
        }
      }
    }
    positions
  }

  /// Checks if the block has reached the bottom
  /// of the board
  pub fn has_landed(&self) -> bool {
    self.get_positions().into_iter()
          .any(|Position {x : _, y}| y == 928.)
  }

  /// Checks if the block collided with any landed
  /// block
  pub fn collides(&self, board: Board) -> bool {
    self.get_positions()
        .into_iter()
        .any(|Position {x, y}|{
          board.cells[(y as usize / 32) + 1 ][x as usize / 32].is_full()
        })
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

    let position: usize = rotation
      .iter()
      .position(|x: &Orientation| *x == self.orientation)
      .unwrap();

    let index: usize =  if position == rotation.len() -1 { 0 } else { position + 1 };
    self.orientation = rotation[index]
  }

  /// Moves the tetromino to the left
  fn move_left(&mut self) -> () {
    let positions: Vec<Position> = self.get_positions();
    if !positions.into_iter()
             .any(|Position {x, y: _}| x == 0. ) {
      self.direction = Direction::L;
      self.position.x -= 32.;
    }
  }

  /// Moves the tetromino to the right
  fn move_right(&mut self) -> () {
    let positions: Vec<Position> = self.get_positions();
    if !positions.into_iter()
             .any(|Position {x, y:_}| x == 352. ) {
      self.direction = Direction::R;
      self.position.x += 32.;
    }
  }

  /// Moves the tetromino to the bottom
  fn move_down(&mut self) -> () {
    self.direction = Direction::D;
    self.position.y += 16.;
  }

  fn drop(&mut self, board: Board) -> () {
    self.direction = Direction::D;
    while !self.has_landed() && !self.collides(board) {
      self.position.y += 1.;
    }
  }

  /// Draws the block onto the canvas
  fn draw(
    &mut self,
    canvas: &mut Canvas,
    ctx: &mut Context ) -> Result<(), GameError> {
      let image = Image::from_path(ctx, "/block.png").unwrap();
      let positions: Vec<Position> = self.get_positions();
      for Position {x, y} in positions {
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